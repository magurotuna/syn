extern crate quote;
extern crate syn;

mod features;

#[macro_use]
mod macros;

use quote::quote;
use syn::{Data, DeriveInput};

#[test]
fn test_unit() {
    let code = quote! {
        struct Unit;
    };

    snapshot!(code as DeriveInput, @r###"
   ⋮DeriveInput {
   ⋮    attrs: [],
   ⋮    vis: Inherited,
   ⋮    ident: Ident(
   ⋮        Unit,
   ⋮    ),
   ⋮    generics: Generics {
   ⋮        lt_token: None,
   ⋮        params: [],
   ⋮        gt_token: None,
   ⋮        where_clause: None,
   ⋮    },
   ⋮    data: Struct(
   ⋮        DataStruct {
   ⋮            struct_token: Struct,
   ⋮            fields: Unit,
   ⋮            semi_token: Some(
   ⋮                Semi,
   ⋮            ),
   ⋮        },
   ⋮    ),
   ⋮}
    "###);
}

#[test]
fn test_struct() {
    let code = quote! {
        #[derive(Debug, Clone)]
        pub struct Item {
            pub ident: Ident,
            pub attrs: Vec<Attribute>
        }
    };

    let actual = {
        snapshot!(code as DeriveInput, @r###"
       ⋮DeriveInput {
       ⋮    attrs: [
       ⋮        Attribute {
       ⋮            pound_token: Pound,
       ⋮            style: Outer,
       ⋮            bracket_token: Bracket,
       ⋮            path: Path {
       ⋮                leading_colon: None,
       ⋮                segments: [
       ⋮                    PathSegment {
       ⋮                        ident: Ident(
       ⋮                            derive,
       ⋮                        ),
       ⋮                        arguments: None,
       ⋮                    },
       ⋮                ],
       ⋮            },
       ⋮            tts: TokenStream [
       ⋮                Group {
       ⋮                    delimiter: Parenthesis,
       ⋮                    stream: TokenStream [
       ⋮                        Ident {
       ⋮                            sym: Debug,
       ⋮                        },
       ⋮                        Punct {
       ⋮                            op: ',',
       ⋮                            spacing: Alone,
       ⋮                        },
       ⋮                        Ident {
       ⋮                            sym: Clone,
       ⋮                        },
       ⋮                    ],
       ⋮                },
       ⋮            ],
       ⋮        },
       ⋮    ],
       ⋮    vis: Public(
       ⋮        VisPublic {
       ⋮            pub_token: Pub,
       ⋮        },
       ⋮    ),
       ⋮    ident: Ident(
       ⋮        Item,
       ⋮    ),
       ⋮    generics: Generics {
       ⋮        lt_token: None,
       ⋮        params: [],
       ⋮        gt_token: None,
       ⋮        where_clause: None,
       ⋮    },
       ⋮    data: Struct(
       ⋮        DataStruct {
       ⋮            struct_token: Struct,
       ⋮            fields: Named(
       ⋮                FieldsNamed {
       ⋮                    brace_token: Brace,
       ⋮                    named: [
       ⋮                        Field {
       ⋮                            attrs: [],
       ⋮                            vis: Public(
       ⋮                                VisPublic {
       ⋮                                    pub_token: Pub,
       ⋮                                },
       ⋮                            ),
       ⋮                            ident: Some(
       ⋮                                Ident(
       ⋮                                    ident,
       ⋮                                ),
       ⋮                            ),
       ⋮                            colon_token: Some(
       ⋮                                Colon,
       ⋮                            ),
       ⋮                            ty: Path(
       ⋮                                TypePath {
       ⋮                                    qself: None,
       ⋮                                    path: Path {
       ⋮                                        leading_colon: None,
       ⋮                                        segments: [
       ⋮                                            PathSegment {
       ⋮                                                ident: Ident(
       ⋮                                                    Ident,
       ⋮                                                ),
       ⋮                                                arguments: None,
       ⋮                                            },
       ⋮                                        ],
       ⋮                                    },
       ⋮                                },
       ⋮                            ),
       ⋮                        },
       ⋮                        Comma,
       ⋮                        Field {
       ⋮                            attrs: [],
       ⋮                            vis: Public(
       ⋮                                VisPublic {
       ⋮                                    pub_token: Pub,
       ⋮                                },
       ⋮                            ),
       ⋮                            ident: Some(
       ⋮                                Ident(
       ⋮                                    attrs,
       ⋮                                ),
       ⋮                            ),
       ⋮                            colon_token: Some(
       ⋮                                Colon,
       ⋮                            ),
       ⋮                            ty: Path(
       ⋮                                TypePath {
       ⋮                                    qself: None,
       ⋮                                    path: Path {
       ⋮                                        leading_colon: None,
       ⋮                                        segments: [
       ⋮                                            PathSegment {
       ⋮                                                ident: Ident(
       ⋮                                                    Vec,
       ⋮                                                ),
       ⋮                                                arguments: AngleBracketed(
       ⋮                                                    AngleBracketedGenericArguments {
       ⋮                                                        colon2_token: None,
       ⋮                                                        lt_token: Lt,
       ⋮                                                        args: [
       ⋮                                                            Type(
       ⋮                                                                Path(
       ⋮                                                                    TypePath {
       ⋮                                                                        qself: None,
       ⋮                                                                        path: Path {
       ⋮                                                                            leading_colon: None,
       ⋮                                                                            segments: [
       ⋮                                                                                PathSegment {
       ⋮                                                                                    ident: Ident(
       ⋮                                                                                        Attribute,
       ⋮                                                                                    ),
       ⋮                                                                                    arguments: None,
       ⋮                                                                                },
       ⋮                                                                            ],
       ⋮                                                                        },
       ⋮                                                                    },
       ⋮                                                                ),
       ⋮                                                            ),
       ⋮                                                        ],
       ⋮                                                        gt_token: Gt,
       ⋮                                                    },
       ⋮                                                ),
       ⋮                                            },
       ⋮                                        ],
       ⋮                                    },
       ⋮                                },
       ⋮                            ),
       ⋮                        },
       ⋮                    ],
       ⋮                },
       ⋮            ),
       ⋮            semi_token: None,
       ⋮        },
       ⋮    ),
       ⋮}
        "###)
    };

    snapshot!(actual.attrs[0].interpret_meta().unwrap(), @r###"
   ⋮List(
   ⋮    MetaList {
   ⋮        ident: Ident(
   ⋮            derive,
   ⋮        ),
   ⋮        paren_token: Paren,
   ⋮        nested: [
   ⋮            Meta(
   ⋮                Word(
   ⋮                    Ident(
   ⋮                        Debug,
   ⋮                    ),
   ⋮                ),
   ⋮            ),
   ⋮            Comma,
   ⋮            Meta(
   ⋮                Word(
   ⋮                    Ident(
   ⋮                        Clone,
   ⋮                    ),
   ⋮                ),
   ⋮            ),
   ⋮        ],
   ⋮    },
   ⋮)
    "###);
}

#[test]
fn test_union() {
    let code = quote! {
        union MaybeUninit<T> {
            uninit: (),
            value: T
        }
    };

    snapshot!(code as DeriveInput);
}

#[test]
#[cfg(feature = "full")]
fn test_enum() {
    let code = quote! {
        /// See the std::result module documentation for details.
        #[must_use]
        pub enum Result<T, E> {
            Ok(T),
            Err(E),
            Surprise = 0isize,

            // Smuggling data into a proc_macro_derive,
            // in the style of https://github.com/dtolnay/proc-macro-hack
            ProcMacroHack = (0, "data").0
        }
    };

    let actual = snapshot!(code as DeriveInput);

    let meta_items: Vec<_> = actual
        .attrs
        .into_iter()
        .map(|attr| attr.interpret_meta().unwrap())
        .collect();

    snapshot!(meta_items);
}

#[test]
fn test_attr_with_path() {
    let code = quote! {
        #[::attr_args::identity
            fn main() { assert_eq!(foo(), "Hello, world!"); }]
        struct Dummy;
    };

    let actual = snapshot!(code as DeriveInput);

    assert!(actual.attrs[0].interpret_meta().is_none());
}

#[test]
fn test_attr_with_non_mod_style_path() {
    let code = quote! {
        #[inert <T>]
        struct S;
    };

    let actual = snapshot!(code as DeriveInput);

    assert!(actual.attrs[0].interpret_meta().is_none());
}

#[test]
fn test_attr_with_mod_style_path_with_self() {
    let code = quote! {
        #[foo::self]
        struct S;
    };

    let actual = snapshot!(code as DeriveInput);

    assert!(actual.attrs[0].interpret_meta().is_none());
}

#[test]
fn test_pub_restricted() {
    // Taken from tests/rust/src/test/ui/resolve/auxiliary/privacy-struct-ctor.rs
    let code = quote! {
        pub(in m) struct Z(pub(in m::n) u8);
    };

    snapshot!(code as DeriveInput);
}

#[test]
fn test_vis_crate() {
    let code = quote! {
        crate struct S;
    };

    snapshot!(code as DeriveInput);
}

#[test]
fn test_pub_restricted_crate() {
    let code = quote! {
        pub(crate) struct S;
    };

    snapshot!(code as DeriveInput);
}

#[test]
fn test_pub_restricted_super() {
    let code = quote! {
        pub(super) struct S;
    };

    snapshot!(code as DeriveInput);
}

#[test]
fn test_pub_restricted_in_super() {
    let code = quote! {
        pub(in super) struct S;
    };

    snapshot!(code as DeriveInput);
}

#[test]
fn test_fields_on_unit_struct() {
    let code = quote! {
        struct S;
    };

    let actual = snapshot!(code as DeriveInput);

    match actual.data {
        Data::Struct(data) => {
            assert_eq!(0, data.fields.iter().count());
        }
        _ => panic!("expected a struct"),
    }
}

#[test]
fn test_fields_on_named_struct() {
    let code = quote! {
        struct S {
            foo: i32,
            pub bar: String,
        }
    };

    let actual = snapshot!(code as DeriveInput);

    match actual.data {
        Data::Struct(data) => {
            snapshot!(data.fields.iter().collect::<Vec<_>>());
        }
        _ => panic!("expected a struct"),
    }
}

#[test]
fn test_fields_on_tuple_struct() {
    let code = quote! {
        struct S(i32, pub String);
    };

    let actual = snapshot!(code as DeriveInput);

    match actual.data {
        Data::Struct(data) => {
            snapshot!(data.fields.iter().collect::<Vec<_>>());
        }
        _ => panic!("expected a struct"),
    }
}

#[test]
fn test_ambiguous_crate() {
    let code = quote! {
        // The field type is `(crate::X)` not `crate (::X)`.
        struct S(crate::X);
    };

    snapshot!(code as DeriveInput);
}
