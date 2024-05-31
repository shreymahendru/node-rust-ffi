import { createRequire } from 'module';
const rust = createRequire(import.meta.url)('../index.node');



let cb = (e) => console.log("from node", e);

rust.initialize_event_aggregator(cb);

console.log("from node", "finished initialize_event_aggregator");

rust.long_async_task_on_native(5)
    .then(() => console.log("done long task"));

rust.get_num_cpu();
rust.dispose_event_aggregator();



// console.log("from node cpus =", rust.get_num_cpu()); // 3

// class Test {
//     constructor() {
//         this.name = "My call name is test";
//     }

//     exec(num) {
//         console.log(this);
//         console.log("from node", this.name, num);
//     }
// }

// const n = 5;

// // const cb = (num) => {
// //     console.log(this);
// //     console.log("from node callback", num);

// // };

// // cb(1000000);

// const test = new Test();
// test.exec(1000000);


// // rust.execute_callback(n, test.exec.bind(test));

// await rust.execute_callback_from_thread(n, test.exec.bind(test));
// console.log("from node", "finished execute_callback_from_thread");

// rust.execute_callback(n, (num) => test.exec(num));

// await rust.long_async_task_on_native(n);
// console.log("from node", "finished long_async_task_on_native");

// await rust.long_async_task_on_worker_thread(n);
// console.log("from node", "finished long_async_task_on_worker_thread");





// // console.log("etf");
// // for (let i = 0; i < 100; i++) {
// //     console.log("from node", i); // 3
// // }

// console.log("done");



