use schema::todos;

#[derive(Deserialize, Insertable)]
#[table_name="todos"]
pub struct NewTodo {
    pub title: String,
    pub item_order: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub item_order: i32,
    pub completed: bool,
}
