#[macro_use]
extern crate introspection_derive;
extern crate introspection;


#[derive(StaticIntrospection)]
struct FrenchToast {
    private_field: u64,
    pub public_field: u8,
}

fn main() {
    use introspection::{ StaticIntrospection };
    println!("Introspection: {:?}", FrenchToast::static_introspection());
}
