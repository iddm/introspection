#[cfg(feature = "serde_support")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "serde_support")]
extern crate serde_json;
extern crate syn;
extern crate quote;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Visibility {
    Public,
    Crate,
    Restricted(String),
    Inherited
}
impl From<syn::Visibility> for Visibility {
    fn from(v: syn::Visibility) -> Visibility {
        use quote::{ Tokens, ToTokens };
        match v {
            syn::Visibility::Public => Visibility::Public,
            syn::Visibility::Crate => Visibility::Crate,
            syn::Visibility::Restricted(path) => {
                let mut tokens = Tokens::new();
                path.to_tokens(&mut tokens);
                Visibility::Restricted(tokens.as_str().to_owned())
            },
            syn::Visibility::Inherited => Visibility::Inherited,
        }
    }
}
impl quote::ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        match *self {
            Visibility::Public => tokens.append("introspection::Visibility::Public"),
            Visibility::Crate => tokens.append("introspection::Visibility::Crate"),
            Visibility::Restricted(ref path) => {
                tokens.append("introspection::Visibility::Restricted(\"");
                tokens.append(&path);
                tokens.append("\".to_owned())");
            },
            Visibility::Inherited => tokens.append("introspection::Visibility::Inherited"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Type {
    Enum,
    Struct,
}
impl quote::ToTokens for Type {
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        match *self {
            Type::Enum => tokens.append("introspection::Type::Enum"),
            Type::Struct => tokens.append("introspection::Type::Struct"),
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct StaticIntrospectionInfo {
    pub ident: String,
    pub visibility: Visibility,
    pub entity_type: Type,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DynamicIntrospectionInfo<'a, T: 'a> {
    pub ident: String,
    pub visibility: Visibility,
    pub entity_type: Type,
    pub fields: Vec<DynamicIntrospectionInfo<'a, T>>,
    pub value: Option<&'a T>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DynamicIntrospectionInfoMut<'a, T: 'a> {
    pub ident: String,
    pub visibility: Visibility,
    pub entity_type: Type,
    pub fields: Vec<DynamicIntrospectionInfo<'a, T>>,
    pub value: Option<&'a mut T>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct IntoDynamicIntrospectionInfo<T: Sized> {
    pub ident: String,
    pub visibility: Visibility,
    pub entity_type: Type,
    pub fields: Vec<IntoDynamicIntrospectionInfo<T>>,
    pub value: Option<T>,
}

pub trait StaticIntrospection {
    /// Provides static introspection information about the type.
    fn static_introspection() -> StaticIntrospectionInfo;
}

pub trait DynamicIntrospection {
    /// Provides read-only runtime introspection information about the object.
    fn dynamic_introspection<'a>(&'a self) -> DynamicIntrospectionInfo<'a, Self>
        where Self: Sized;
    /// Provides mutable runtime introspection information about the object.
    fn dynamic_introspection_mut<'a>(&'a mut self) -> DynamicIntrospectionInfoMut<'a, Self>
        where Self: Sized;
}

pub trait IntoIntrospection {
    /// Consumes the object and returns it inside the introspection.
    fn into_introspection(mut self) -> IntoDynamicIntrospectionInfo<Self>
        where Self: Sized;
}
