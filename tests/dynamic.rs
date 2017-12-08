#[macro_use]
extern crate introspection_derive;
extern crate introspection;

#[test]
fn dynamic_introspection_simple() {
    use introspection::{ DynamicIntrospection, Visibility, Type };

    #[derive(Debug, Copy, Clone, DynamicIntrospection, Eq, PartialEq)]
    struct FrenchToast {
        private_field: u64,
        pub public_field: u8,
    }

    let mut french_toast = FrenchToast {
        private_field: 5u64,
        public_field: 6u8,
    };
    let mut copy_french_toast = french_toast.clone();
    let mut dynamic_introspection = french_toast.dynamic_introspection_mut();

    assert_eq!(dynamic_introspection.ident, "FrenchToast");
    assert_eq!(dynamic_introspection.visibility, Visibility::Inherited);
    assert_eq!(dynamic_introspection.entity_type, Type::Struct);
    assert_eq!(dynamic_introspection.fields, vec![]);
    assert_eq!(dynamic_introspection.value, Some(&mut copy_french_toast));
}
