pub struct Schema {
    pub rels: Vec<Relation>,
}

pub enum Relation {
    Table(Table),
    Join(Join),
    View(View),
}

pub struct Table {
    pub name: String,
    pub columns: TableColumn,
}

pub struct TableColumn {
    pub name: String,
    pub unique_key: bool,
    pub foreign_key: Option<Path>,
}

pub struct Path {
    pub pieces: Vec<String>,
}

pub struct Join {
    pub name: String,
}

pub struct View {
    pub name: String,
}
