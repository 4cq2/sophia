# Decode from JSON

## destream\_json

~~~
D:\rust\bin\cargo.exe add --no-default-features destream_json
25      pin-project-internal v1.0.12 (proc-macro)
~~~

## display\_json

~~~
D:\rust\bin\cargo.exe add --no-default-features display_json
9       ryu v1.0.11
~~~

## humphrey\_json

~~~
cargo.exe add --no-default-features humphrey_json
1       humphrey_json v0.2.2

cargo.exe add --no-default-features --features derive humphrey_json
1       humphrey_json v0.2.2
6       syn v1.0.100
~~~

## ijson

~~~
D:\rust\bin\cargo.exe add --no-default-features ijson
8       ryu v1.0.11
~~~

## json

Is this project abandoned?

https://github.com/maciejhirsz/json-rust/issues/205

## json-codec

Indent

https://gitlab.com/twittner/json-codec/-/issues/1

## json-commons

this is just a wrapper for another package:

https://github.com/Lucas-Palomo/json-commons/issues/1

## json\_flex

`to_json` with indent

<https://github.com/nacika-ins/json_flex/issues/10>

## json\_minimal

print with indent

<https://github.com/36den/json_minimal-rust/issues/9>

## json\_stream

publish crate

<https://github.com/alexmaco/json_stream/issues/1>

## justjson

`as_str` behavior is undesired

https://github.com/khonsulabs/justjson/issues/11

## lite-json

How to index

https://github.com/xlc/lite-json/issues/27

## nop-json

~~~
D:\rust\bin\cargo.exe add --no-default-features nop-json
6       numtoa v0.2.4
~~~

## parson

object creation is difficult

https://github.com/zS1L3NT/rs-parson/issues/4

## qj

publish crate

https://github.com/deependapp/quick-json/issues/1

## rust\_json

~~~
D:\rust\bin\cargo.exe add --no-default-features rust_json
1       rust_json v0.1.4

D:\rust\bin\cargo.exe add --no-default-features rust_json
D:\rust\bin\cargo.exe add --no-default-features rust_json_derive
1       rust_json v0.1.4
2       rust_json_derive v0.1.1 (proc-macro)
6       syn v1.0.100
~~~

## serde-json-core

Non-features
Anything that involves dynamic memory allocation
Like the dynamic Value type

## serde\_json

~~~
cargo.exe add --no-default-features --features std serde_json
1       serde_json v1.0.85
4       serde v1.0.144

cargo.exe add --no-default-features --features serde_derive serde
cargo.exe add --no-default-features --features std serde_json
1       serde v1.0.144
7       serde_json v1.0.85
9       ryu v1.0.11
~~~

## simd-json

~~~
D:\rust\bin\cargo.exe add --no-default-features simd-json
14      ryu v1.0.11
~~~

## simple\_json

`to_string` with indent

https://github.com/Rafagd/json-rs/issues/4

## sj

~~~
cargo.exe add --no-default-features sj
1       sj v0.23.0
~~~

## tinyjson

this is ugly:

~~~
use tinyjson::JsonValue;

fn kv(s: impl Into<String>, v: impl Into<JsonValue>) -> (String, JsonValue) {
    (s.into(), v.into())
}

fn main() {
   let value = JsonValue::Object([
      kv("null", ()),
      kv("boolean", true),
      kv("number", 1.0),
      kv("string", "one".to_string()),
   ].into());
   dbg!(value);
}
~~~

https://github.com/rhysd/tinyjson

## tumekiri

publish crate

https://github.com/k-nasa/tumekiri/issues/1
