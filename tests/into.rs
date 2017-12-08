#[macro_use]
extern crate introspection_derive;
extern crate introspection;

#[test]
fn into_dynamic_introspection_simple() {
    use introspection::{ IntoIntrospection, Visibility, Type };

    #[derive(Debug, Copy, Clone, IntoIntrospection, Eq, PartialEq)]
    struct FrenchToast {
        private_field: u64,
        pub public_field: u8,
    }

    let french_toast = FrenchToast {
        private_field: 5u64,
        public_field: 6u8,
    };
    let copy_french_toast = french_toast.clone();
    let mut dynamic_introspection = french_toast.into_introspection();

    assert_eq!(dynamic_introspection.ident, "FrenchToast");
    assert_eq!(dynamic_introspection.visibility, Visibility::Inherited);
    assert_eq!(dynamic_introspection.entity_type, Type::Struct);
    assert_eq!(dynamic_introspection.fields, vec![]);
    assert_eq!(dynamic_introspection.value, Some(copy_french_toast));
}
