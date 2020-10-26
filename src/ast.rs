#[derive(Debug)]
pub struct Schema {
    pub rels: Vec<Relation>,
}

#[derive(Debug)]
pub enum Relation {
    Table(Table),
    Join(Join),
    View(View),
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<TableColumn>,
}

#[derive(Debug)]
pub struct TableColumn {
    pub name: String,
    pub type_: String,
    pub prop: Option<TableColumnProp>,
}

#[derive(Debug)]
pub enum TableColumnProp {
    UniqueKey,
    ForeignKey(Path),
}

#[derive(Debug)]
pub struct Path {
    pub table: String,
    pub column: String,
}

#[derive(Debug)]
pub struct Join {
    pub name: String,
}

#[derive(Debug)]
pub struct View {
    pub name: String,
}
