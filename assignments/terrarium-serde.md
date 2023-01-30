In this exercise, we will implement a mock JSON API.

You will learn:

-   How to validate JSON using `serde_json::Value`

-   How to describe a JSON structure in Rust types

Task
====

1.  Create a new Terrarium project

2.  Send the following JSON as a POST request

        {
            "name": "John Doe",
            "age": 43,
            "address": {
                "street": "10 Downing Street",
                "city": "London"
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }

3.  Validate the JSON structure to have all those fields

4.  On success, return the structure with a `200 OK`

5.  On error, return status `422`, along with the resulting error

Extension Task
==============

1.  Make it optional to send the Address

Helpers
=======

Sending JSON via curl
---------------------

Copy the above JSON data to a file, then send the file using:

    curl -XPOST --data-binary @my_json_file.json $URL

Deserializing and Serializing JSON
----------------------------------

Use [SerDe](https://serde.rs):

    extern crate serde;

    #[macro_use]
    extern crate serde_derive;
    extern crate serde_json;

SerDe JSON allows either deserializing into a generic JSON value:

    let parsed_person = serde_json::from_slice::<serde_json::Value>(req.body());

Or you can define a structure to be serializable and deserializable:

    #[derive(Serialize,Deserialize)]
    struct MyType {
        field1: String,
        field2: u32
    }

    let parsed_person = serde_json::from_slice::<MyType>(req.body());
