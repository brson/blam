use crate::ast::Schema;
use anyhow::{Result, anyhow};
use self::parser::SchemaParser;

lalrpop_mod!(pub parser);

pub fn parse(schema: &str) -> Result<Schema> {
    let parser = SchemaParser::new();
    let ast = parser.parse(schema);
    let ast = ast.map_err(|e| anyhow!("{}", e));
    Ok(ast?)
}
