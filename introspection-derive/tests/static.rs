#[macro_use]
extern crate introspection_derive;
extern crate introspection;

#[test]
fn it_adds_two() {
    use introspection::{ Introspection };

    #[derive(Introspection)]
    struct FrenchToast {
        private_field: u64,
        pub public_field: u8,
    }

    println!("Introspection: {:?}", FrenchToast::introspection());
    assert_eq!(4, 4);
}
