#[macro_use]
extern crate introspection_derive;
extern crate introspection;

#[test]
fn static_introspection_simple() {
    use introspection::{ StaticIntrospection, Visibility, Type };

    #[derive(StaticIntrospection)]
    struct FrenchToast {
        #[allow(unused)]
        private_field: u64,
        #[allow(unused)]
        pub public_field: u8,
    }

    let static_introspection = FrenchToast::static_introspection();

    assert_eq!(static_introspection.ident, "FrenchToast");
    assert_eq!(static_introspection.visibility, Visibility::Inherited);
    assert_eq!(static_introspection.entity_type, Type::Struct);
    assert_eq!(static_introspection.fields, vec!["private_field", "public_field"]);
}
