#[macro_use]
extern crate introspection_derive;
extern crate introspection;


#[derive(Introspection)]
struct FrenchToast {
    private_field: u64,
    pub public_field: u8,
}

fn main() {
    use introspection::{ Introspection };
    println!("Introspection: {:?}", FrenchToast::introspection());
}
