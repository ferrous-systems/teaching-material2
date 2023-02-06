[Table of Contents](./index.html)

Example: JavaScript
===================

-   Eager execution model

-   Other languages behave similarly (C\#, Python, Swift, Kotlin, etc.)

-   JavaScript calls its futures "Promises"

<!-- -->

    async fn subtask() {
        println!("> > subtask"); // 
    }

    async fn task() {
        println!("> Before subtask"); // 
        subtask().await;
        println!("> After subtask"); // 
    }

    fn main() {
        futures::executor::block_on(async {
            println!("before future"); // 
            let future = task();
            println!("future is created"); // 
            future.await;
            println!("future is awaited"); // 
        });
    }

    async function subtask() {
      console.log("> > subtask"); // 
    }

    async function task() {
      console.log("> before subtask"); // 
      await subtask();
      console.log("> after subtask"); // 
    }

    (async function main() {
      console.log("before promise"); // 
      let promise = task();
      console.log("promise is created"); // 
      await promise;
      console.log("promise is awaited"); // 
    })();

    async function subtask() {
      console.log("> > subtask"); // 
    }

    async function task() {
      console.log("> before subtask"); // 
      await subtask();
      console.log("> after subtask"); // 
    }

    (async function main() {
      console.log("before promise"); // 
      let promise = task();
      console.log("promise is created"); // 
      await promise;
      console.log("promise is awaited"); // 
    })();

    sequenceDiagram
      participant main
      participant task
      participant subtask
      Note right of main: console.log("before promise")
      main->>task: let promise = task()
      Note right of task: console.log("> before subtask")
      task->>subtask: subtask()
      Note right of subtask: console.log("> > subtask")
      subtask-->>task: a promise is prepared
      task-->>main: a promise is prepared
      Note right of main: console.log("promise is created")
      main->>task: await
      task->>subtask: await
      subtask-->>task: return
      Note right of task: console.log("> after subtask")
      task-->>main: return
      Note right of main: console.log("promise is awaited")

    async function subtask() {
      console.log("> > subtask"); // 
    }

    async function task() {
      console.log("> before subtask"); // 
      await subtask();
      console.log("> after subtask"); // 
    }

    (async function main() {
      console.log("before promise"); // 
      let promise = task();
      console.log("promise is created"); // 
      await promise;
      console.log("promise is awaited"); // 
    })();

Output: JavaScript

    before promise

    > before subtask

    > > subtask

    promise is created

    > after subtask

    promise is awaited

Eager Execution: Takeaways
==========================

-   as soon as async function is called it starts executing

-   runs till the first `await` point

-   inner async functions run their code, too, and stop at `await`

-   an async function with no `await` inside will execute its full body
    eagerly

<!-- -->

    async fn subtask() {
        println!("> > subtask"); // 
    }

    async fn task() {
        println!("> Before subtask"); // 
        subtask().await;
        println!("> After subtask"); // 
    }

    fn main() {
        futures::executor::block_on(async {
            println!("before future"); // 
            let future = task();
            println!("future is created"); // 
            future.await;
            println!("future is awaited"); // 
        });
    }

    sequenceDiagram
      participant main
      participant task
      participant subtask
      Note right of main: println!("before future")
      Note right of main: let future = task()
      Note right of main: println!("future is created")
      main->>task: await
      Note right of task: println!("> Before subtask")
      Note right of task: subtask()
      task->>subtask: await
      Note right of subtask: println!("> > subtask")
      subtask-->>task: return
      Note right of task: println!("> After subtask")
      task-->>main: return
      Note right of main: println!("future is awaited")

    async fn subtask() {
        println!("> > subtask"); // 
    }

    async fn task() {
        println!("> Before subtask"); // 
        subtask().await;
        println!("> After subtask"); // 
    }

    fn main() {
        futures::executor::block_on(async {
            println!("before future"); // 
            let future = task();
            println!("future is created"); // 
            future.await;
            println!("future is awaited"); // 
        });
    }

Output: Rust

    before future

    future is created

    > Before subtask

    > > subtask

    > After subtask

    future is awaited

    async fn subtask() {
        println!("> > subtask"); // 
    }

    async fn task() {
        println!("> Before subtask"); // 
        subtask().await;
        println!("> After subtask"); // 
    }

    fn main() {
        futures::executor::block_on(async {
            println!("before future"); // 
            let future = task();
            println!("future is created"); // 
            future.await;
            println!("future is awaited"); // 
        });
    }

Output: Rust

    before future

    future is created

    > Before subtask

    > > subtask

    > After subtask

    future is awaited

Output: JavaScript

    before promise

    > before subtask

    > > subtask

    promise is created

    > after subtask

    promise is awaited

    async function subtask() {
      console.log("> > subtask"); // 
    }

    async function task() {
      console.log("> before subtask"); // 
      await subtask();
      console.log("> after subtask"); // 
    }

    (async function main() {
      console.log("before promise"); // 
      let promise = task();
      console.log("promise is created"); // 
      await promise;
      console.log("promise is awaited"); // 
    })();

Lazy Future Execution: Takeaways
================================

-   no code is being run until a future is `await` ed

-   `await` triggers the whole chain to execute

-   no "fire now, `await` later" workflow
