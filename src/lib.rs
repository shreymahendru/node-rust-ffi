use std::{thread, time::Duration};

use neon::{
    context::{Context, FunctionContext, ModuleContext}, result::{JsResult, NeonResult}, types::{JsFunction, JsNumber, JsObject, JsPromise, JsString, JsUndefined, JsValue}
};
use neon::object::Object;


fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn get_num_cpu(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let count = num_cpus::get() as f64;
    Ok(cx.number(count))
}

fn long_async_task_on_native(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let count = cx.argument::<JsNumber>(0)?.value(&mut cx);
    println!("count will spawn thread: {}", count);

    let (deferred, promise) = cx.promise();

    let channel = cx.channel();
    std::thread::spawn(move || {
        for i in 0..count as i32 {
            println!("running in thread i: {}", i);
            thread::sleep(Duration::from_secs(1));
        }

        deferred.settle_with(&channel, |mut cx| Ok(cx.undefined()))
    });

    Ok(promise)
}

fn long_async_task_on_worker_thread(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let count = cx.argument::<JsNumber>(0)?.value(&mut cx);

    let promise = cx
        .task(move || {
            for i in 0..count as i32 {
                println!("running in thread i: {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        })
        .promise(move |mut cx, _| Ok(cx.undefined()));

    Ok(promise)
}

fn execute_callback(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    print!("execute_callback");
    let count: f64 = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let callback = cx.argument::<JsFunction>(1)?;

    // let this = cx.this_value();
    let this = cx.undefined();

    for i in 0..count as i32 {
        println!("running in thread i: {}", i);

        let args = vec![cx.number(i).upcast::<JsValue>()];

        callback.call(&mut cx, this, args)?;
    }

    Ok(cx.undefined())
}

fn execute_callback_from_thread(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let count = cx.argument::<JsNumber>(0)?.value(&mut cx);

    let mut callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let mut this = cx.this::<JsObject>()?.root(&mut cx);

    println!("count will spawn thread: {}", count);

    let (deferred, promise) = cx.promise();

    let channel = cx.channel();
    std::thread::spawn(move || {

        for i in 0..count as i32 {

            println!("running in thread i: {}", i);


            let handle = channel.send(move |mut cx| {
            let args = vec![cx.number(i).upcast::<JsValue>()];
            // let this = cx.undefined();

                let cb = callback.clone(&mut cx).into_inner(&mut cx);
                let t = this.clone(&mut cx).into_inner(&mut cx);

                cb.call(&mut cx, t, args)?;

                Ok((this , callback))
            });

            (this, callback) = handle.join().unwrap();

            thread::sleep(Duration::from_secs(1));
        }

        deferred.settle_with(&channel, |mut cx| Ok(cx.undefined()))
    });

    Ok(promise)
}



// fn async_fibonacci(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     // These types (`f64`, `Root<JsFunction>`, `Channel`) may all be sent
//     // across threads.
//     let n = cx.argument::<JsNumber>(0)?.value(&mut cx);
//     let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
//     let channel = cx.channel();

//     // Spawn a thread to complete the execution. This will _not_ block the
//     // JavaScript event loop.
//     std::thread::spawn(move || {
//         let result = fibonacci(n);

//         // Send a closure as a task to be executed by the JavaScript event
//         // loop. This _will_ block the event loop while executing.
//         channel.send(move |mut cx| {
//             let callback = callback.into_inner(&mut cx);
//             let this = cx.undefined();
//             let args = vec![
//                 cx.null().upcast::<JsValue>(),
//                 cx.number(result).upcast(),
//             ];

//             callback.call(&mut cx, this, args)?;

//             Ok(())
//         });
//     });

//     Ok(cx.undefined())
// }


// fn fibonacci(n: f64) -> f64 {
//     if n <= 1.0 {
//         return n;
//     }

//     fibonacci(n - 1.0) + fibonacci(n - 2.0)
// }

// fn long_task(mut cx: FunctionContext) -> JsResult<JsPromise> {
//     let count = cx.argument::<JsNumber>(0)?.value(&mut cx);
//     println!("count will spawn thread: {}", count);

//     let (deferred, promise) = cx.promise();

//     // let (sender, receiver) = mpsc::channel();

//     // let v = receiver.recv().unwrap();
//     // println!("received: {:?}", v);

//     let channel = cx.channel();
//     std::thread::spawn(move || {
//         for i in 0..count as i32 {
//             println!("running in thread i: {}", i);
//             thread::sleep(Duration::from_secs(1));
//         }

//         // sender.send(true).unwrap();

//         deferred.settle_with(&channel, |mut cx| Ok(cx.undefined()))
//     });

//     // deferred.settle_with(&channel, |mut cx| {
//     //     Ok(cx.undefined())
//     // });

//     Ok(promise)
// }

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("get_num_cpu", get_num_cpu)?;
    cx.export_function("long_async_task_on_native", long_async_task_on_native)?;
    cx.export_function(
        "long_async_task_on_worker_thread",
        long_async_task_on_worker_thread,
    )?;

    cx.export_function("execute_callback", execute_callback)?;

    cx.export_function("execute_callback_from_thread", execute_callback_from_thread)?;
    Ok(())
}
