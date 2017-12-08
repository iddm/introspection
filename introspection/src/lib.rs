#[cfg(feature = "serde_support")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "serde_support")]
extern crate serde_json;
extern crate syn;
extern crate quote;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct StaticIntrospectionInfo {
    pub ident: String,
    pub visibility: Visibility,
    pub entity_type: Type,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DynamicIntrospectionInfo<'a, T> {
    pub ident: String,
    pub visibility: Visibility,
    pub entity_type: Type,
    pub fields: Vec<DynamicIntrospectionInfo<'a, T>>,
    pub value: Option<&'a T>,
}

pub trait Introspection {
    fn static_introspection() -> StaticIntrospectionInfo;
    fn dynamic_introspection(&self) -> &DynamicIntrospectionInfo<Self>;
    fn dynamic_introspection_mut(&mut self) -> &mut DynamicIntrospectionInfo<Self>;
}
