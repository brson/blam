use anyhow::Result;
use crate::ast::{Schema, Relation};
use std::collections::BTreeSet;

pub fn check(ast: &Schema) -> Result<TypeInfo> {
    let relation_names = get_relation_names(ast);

    panic!()
}

#[derive(Debug)]
pub struct TypeInfo;

fn get_relation_names(ast: &Schema) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    for rel in &ast.rels {
        let name = match rel {
            Relation::Table(r) => &r.name,
            Relation::Join(r) => &r.name,
            Relation::View(r) => &r.name,
        };
        let name = name.to_string();
        set.insert(name);
    }
    set
}
