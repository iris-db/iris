use std::sync::Mutex;

use rocket::State;
use rocket_contrib::json::Json;
use serde_json::json;

use crate::graph::database::Database;
use crate::graph::node::CreateNodeData;
use crate::lib::bson::JsonObject;

#[post("/<plane>", data = "<body>")]
fn mutate_plane(
    plane: String,
    body: Json<JsonObject>,
    db: State<Mutex<Database>>,
) -> Json<JsonObject> {
    let mut db = db.inner().lock().unwrap();
    let planes = db.planes_mut();
    let p = &mut planes[0];

    let data = body.0;
    for k in data.keys() {
        if k.eq("$insert") {
            let v = data.get(k).unwrap();

            let mut d = Vec::new();
            d.push(CreateNodeData(Some(v.to_string()), None));

            p.insert_nodes(Some(d));
        }
    }

    Json(json!({ "hello": plane }).as_object().unwrap().clone())
}

pub fn start_rest_server() {
    let db = Database::new();

    rocket::ignite()
        .mount("/", routes![mutate_plane])
        .manage(Mutex::new(db))
        .launch();
}
