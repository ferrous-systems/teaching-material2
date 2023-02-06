Let’s refactor our async application a little bit. Instead of using a
Mutex to moderate our queue, let’s instead use a single task and use a
channel for synchronisation.

Task
====

1.  Pick a MPMC channel module (recommended tokio, async-std or
    async-channel)

2.  Implement a function `mailbox` that loops over all incoming messages
    and

    -   On "PUBLISH", just stores data

    -   On "RETRIEVE", sends the message back to the task requesting the
        data

3.  Make sure this function gets started correctly

you will need to implement 2 channels, one towards your mailbox and one
backchannel.

Getting started
===============

Use this template:

    async fn mailbox_function(storage: VecDeque<String>, recv: Receiver</* */>) {
      // ...
    }
