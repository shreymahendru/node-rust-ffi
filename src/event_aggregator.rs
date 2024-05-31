use std::sync::{Arc, OnceLock, RwLock};

use neon::{
    context::{Context, FunctionContext},
    event::Channel,
    handle::Root,
    object::Object,
    result::JsResult,
    types::{JsFunction, JsObject, JsUndefined, JsValue},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    Log { message: String },
    OtherMessage { num: i32, description: String },
}

pub static EVENT_AGGREGATOR: OnceLock<EventAggregator> = OnceLock::new();

#[derive(Debug)]
pub struct EventAggregator {
    _channel: RwLock<Option<Channel>>,
    _js_this: Arc<Root<JsObject>>,
    _callback: Arc<Root<JsFunction>>,
}

impl EventAggregator {
    pub fn instance() -> &'static EventAggregator {
        EVENT_AGGREGATOR
            .get()
            .expect("EventAggregator is not initialized")
    }

    fn new(channel: Channel, callback: Root<JsFunction>, js_this: Root<JsObject>) -> Self {
        EventAggregator {
            _channel: RwLock::new(Some(channel)),
            _callback: Arc::new(callback),
            _js_this: Arc::new(js_this),
        }
    }

    pub fn publish(&self, event: Event) {
        print!("Will publish {:#?}", event);

        // let serialized = serde_json::to_string(&event);
        let serialized = serde_json::to_string(&event)
            .expect(format!("Failed to serialize event: {:#?}", event).as_str());

        let callback = self._callback.clone();
        let js_this = self._js_this.clone();

        let binding = self._channel.read().unwrap();

        let channel = binding
            .as_ref()
            .expect("Event Aggregator is not initialized.");

        channel.send(move |mut cx| {
            let callback = callback.as_ref().clone(&mut cx).into_inner(&mut cx);
            let js_this = js_this.as_ref().clone(&mut cx).into_inner(&mut cx);
            let event = cx.string(serialized);
            let args = [event.upcast::<JsValue>()];

            callback.call(&mut cx, js_this, args)?;

            Ok(())
        });
    }

    fn _dispose(&self) {
        println!("Dropping EventAggregator");
        self._channel.write().unwrap().take();
    }
}

pub fn initialize_event_aggregator(mut context: FunctionContext) -> JsResult<JsUndefined> {
    let channel = context.channel();
    let callback = context.argument::<JsFunction>(0)?.root(&mut context);
    let js_this = context.this::<JsObject>()?.root(&mut context);

    EVENT_AGGREGATOR
        .set(EventAggregator::new(channel, callback, js_this))
        .expect("Failed in to initialize EventAggregator");

    Ok(context.undefined())
}

pub fn dispose_event_aggregator(mut context: FunctionContext) -> JsResult<JsUndefined> {
    println!("Disposing EventAggregator");
    EventAggregator::instance()._dispose();
    Ok(context.undefined())
}
