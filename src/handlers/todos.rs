use diesel;
use diesel::prelude::*;
use hyper;

use framework::prelude::*;

use models::todos::{NewTodo,Todo};

pub fn create(req: Request) -> impl Future<Item=impl Responder, Error=Error> {
    use schema::todos;

    let conn = req.pool().get().expect("Could not obtain connection");
    req.parse_json::<NewTodo>()
        .and_then(move |new_todo| {
            let todo = diesel::insert(&new_todo)
                .into(todos::table)
                .get_result::<Todo>(&*conn)?;
            Ok(Json(todo)
               .respond()
               .with_status(hyper::StatusCode::Created))
        })
}

pub fn list(req: Request) -> Result<impl Responder>
{
    use schema::todos::dsl::*;

    let conn = req.pool().get().expect("Could not obtain connection");
    let todo_list = todos
       .limit(5)
       .load::<Todo>(&*conn)?;
    Ok(Json(todo_list))
}

