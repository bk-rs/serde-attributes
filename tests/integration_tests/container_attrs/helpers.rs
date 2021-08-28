use syn::{parse_str, DeriveInput, Meta, MetaList, NestedMeta};

pub fn parse_serde_meta(input: &str) -> Meta {
    let attrs = parse_str::<DeriveInput>(input).unwrap().attrs;
    match attrs[0].parse_meta().unwrap() {
        Meta::List(MetaList {
            path,
            paren_token: _,
            nested: _,
        }) if path.is_ident("derive") => {}
        meta => {
            println!("{:?}", meta);
            panic!()
        }
    }
    match attrs[1].parse_meta().unwrap() {
        Meta::List(MetaList {
            path,
            paren_token: _,
            nested,
        }) if path.is_ident("serde") => match nested.first().cloned() {
            Some(NestedMeta::Meta(meta)) => meta,
            _ => panic!(),
        },
        meta => {
            println!("{:?}", meta);
            panic!()
        }
    }
}
