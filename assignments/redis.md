In this exercise, we will implement a simple redis client.

-   [Presentation slides](../redis.html)

Preparation + Setup
===================

Step 1  
Install a Redis server on your machine, using e.g. your favorite package
manager or Docker.

Step 2  
Create a binary project called `redis-client`:
`cargo new --bin redis-client`

Implementation
==============

Step 3  
Implement a command line handler that lets your program either

-   get and print the value for a given key, or

-   set a given value for a given key, and

-   defaults to `redis://127.0.0.1/` as the server to connect to, but
    lets you override that

You can launch your binary with
`cargo run \-- your args --and-params=here`. Test argument parsing works
as expected using `println!`.

Step 4  
Add a library crate to your package and implement a `RedisClient` struct
that can get and set values. It should be constructable using only a
generic `IntoConnectionInfo` parameter and expose no redis-specific
functionality (essentially behave like a dictionary).

Test its functionality with a round trip unit test (set `key` to
`value`, assert `get(key) == value`).

Step 5  
Hook up your command line arguments to the `RedisClient`. In the `get`
case, `println!` a String created from the returned value.

Solution
========

<https://github.com/ferrous-systems/teaching-material/tree/main/assignments/solutions/redis>
