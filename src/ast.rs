pub struct Schema {
    pub rels: Vec<Relation>,
}

pub enum Relation {
    Table(Table),
    Join(Join),
    View(View),
}

pub struct Table {
    name: String,
    columns: TableColumn,
}

pub struct TableColumn {
    name: String,
    unique_key: bool,
    foreign_key: Option<Path>,
}

pub struct Path {
    pieces: Vec<String>,
}

pub struct Join {
    name: String,
}

pub struct View {
    name: String,
}
