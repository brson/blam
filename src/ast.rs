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
    pub type_: Type,
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
    pub from: String,
    pub with: Vec<JoinWith>,
    pub where_: Vec<JoinWhere>,
}

#[derive(Debug)]
pub struct JoinPreds {
    pub from: String,
    pub with: Vec<JoinWith>,
    pub where_: Vec<JoinWhere>,
}

#[derive(Debug)]
pub struct JoinWith {
    pub name: String,
    pub rename: Option<String>,
}

#[derive(Debug)]
pub struct JoinWhere {
    pub from_name: Path,
    pub with_name: Path,
}

#[derive(Debug)]
pub struct View {
    pub name: String,
    pub from: String,
    pub columns: Vec<ViewColumn>,
}

#[derive(Debug)]
pub struct ViewColumn {
    pub name: String,
    pub from_name: Path,
}

#[derive(Debug)]
pub enum Type {
    Integer,
    Float,
    String,
    Date,
}
