# introspection
A rust introspection procedural macro.


[![](https://meritbadge.herokuapp.com/introspection)](https://crates.io/crates/introspection) [![](https://travis-ci.org/vityafx/introspection.svg?branch=master)](https://travis-ci.org/vityafx/introspection)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


## What does it do?

It simply converts code from compile stage (from `syn` crate) to simplier structs. Works for `struct`s and `enum`s.
 You may obtain this information through a [`Introspection`](https://github.com/vityafx/introspection/blob/master/introspection/src/lib.rs) trait.
  

## Usage

1. Add `introspection` as dependency in your `Cargo.toml`:

 ```toml
 [dependencies]
 introspection-derive = "0.1"
 introspection = "0.1"
 ```

2. Create a struct or enum:

 ```rust
 #[macro_use]
 extern crate introspection_derive;
 extern crate introspection;
 
 
 #[derive(Introspection)]
 struct FrenchToast {
     private_field: u64,
     pub public_field: u8,
 }

 ```

3. Use it:

 ```rust
 fn main() {
    use introspection::{ Introspection };
    println!("Introspection: {:?}", FrenchToast::introspection());
 }
 ```
 
4. See the results:
````
      Running `target/debug/introspection-test`
Introspection: IntrospectionInfo { ident: "FrenchToast", visibility: Inherited, entity_type: Struct, fields: ["private_field", "public_field"] }
```
 
P.S. Personally I doubt this crate will help someone because it is impossible to do a lot of interesting and useful stuff from procedural macro at this moment unfortunately (rustc v1.15).
 
## License

This project is [licensed under the MIT license](https://github.com/vityafx/introspection/blob/master/LICENSE).
