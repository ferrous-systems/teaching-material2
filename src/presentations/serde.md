[Table of Contents](./index.html)

**Ser**ialization and **De**serialization
----

<https://serde.rs/>

`Serialize` & `Deserialize`
----

To make a Rust structure (de)serializable:

    #[macro_use]

    #[derive(Debug, Serialize, Deserialize)]
    struct Move {
        id: usize,
        direction: Direction,
    }

    #[derive(Debug, Serialize, Deserialize)]
    enum Direction { North, South, East, West }

    fn main() {}

Formats
----

Serde supports a number of formats, such as:

-   JSON

-   CBOR

-   YAML

-   TOML

-   BSON

-   MessagePack

-   … More!

Did you enjoy that acronym salad?

`Serialize`
----

To JSON:

    #[macro_use]

    use Direction::*;

    fn main() {
        let action = Move { id: 1, direction: West };
        let payload = serde_json::to_string(&action);
        println!("{:?}", payload);
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Move {
        id: usize,
        direction: Direction,
    }

    #[derive(Debug, Serialize, Deserialize)]
    enum Direction { North, South, East, West }

`Deserialize`
----

From JSON:

    fn main() {
        let payload = "{\"id\":1,\"direction\":\"West\"}";
        let action = serde_json::from_str::<Move>(&payload);
        println!("{:?}", action);
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Move {
        id: usize,
        direction: Direction,
    }

    #[derive(Debug, Serialize, Deserialize)]
    enum Direction { North, South, East, West }

Transcode
----

    use serde_transcode::transcode;

    fn main() {
        let payload = "{\"id\":1,\"direction\":\"West\"}";
        let mut buffer = String::new();
        {
            let mut ser = toml::Serializer::new(&mut buffer);
            let mut de = serde_json::Deserializer::from_str(&payload);
            transcode(&mut de, &mut ser)
                .unwrap();
        }
        println!("{:?}", buffer);
    }

Attributes
----

`serde` has a large number of attributes you can utilize:

    #[serde(deny_unknown_fields)] // Be extra strict
    struct Move {
        #[serde(default)] // Call usize::default()
        id: usize,
        #[serde(rename = "dir")] // Use a different name
        direction: Direction,
    }

<https://serde.rs/attributes.html>
