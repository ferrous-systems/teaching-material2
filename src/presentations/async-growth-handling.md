[Table of Contents](./index.html)

Information gathering
----

Your project is dynamic:

-   Make it traceable

-   Constantly monitor

Backpressure
----

Backpressure is the process of "pushing back" on producers that are to
fast.

This throttles the system, but allows it to not fail.

Bounded vs. unbounded growth
----

-   bounded channels are predictable

-   unbounded are more dynamic

-   bounded provide backpressure

-   unbounded are useful if you know they are never beyond a certain
    size

Holding state
----

-   Tasks can hold state

-   Otherwise, Mutexes and RWlocks allow sharing

Dropping futures
----

-   Dropping a futures means cancelling it

-   Be aware of what happens if it is cancelled

Shutting down
----

-   Make sure your signal handling is centralized

-   Every component should subscribe to a cancel notification

Implementing a custom Future: Pinning
----

-   Futures are not allowed to move in Memory

-   The type that describes that is called `Pin`

-   Pinning is hard, but there’s support libraries

The poll protocol
----

Futures are `poll` based - that means they get asked if they are
complete.

-   This happens an infinite number of times, until they mark themselves
    complete

-   The process is optimised through the `Waker` type

-   Implementing poll yourself is rather easy
