// use crate::db::schema::items;
// use crate::models::item::model::{Item, NewItem};
// use crate::models::wz;
//
// pub async fn get_item_by_id(state: SharedState, id: i32) -> QueryResult<Item> {
//     let db = {
//         let state = state.lock().await;
//         state.db.clone()
//     };
//     let mut conn = db.get().map_err(|e| {
//         diesel::result::Error::DatabaseError(
//             diesel::result::DatabaseErrorKind::UnableToSendCommand,
//             Box::new(e.to_string()),
//         )
//     })?;
//     items::table
//         .filter(items::id.eq(id))
//         .load::<Item>(&mut conn)
// }
//
// pub async fn create_item(state: SharedState, item: NewItem) -> QueryResult<Item> {
//     let db = {
//         let state = state.lock().await;
//         state.db.clone()
//     };
//     let mut conn = db.get().map_err(|e| {
//         diesel::result::Error::DatabaseError(
//             diesel::result::DatabaseErrorKind::UnableToSendCommand,
//             Box::new(e.to_string()),
//         )
//     })?;
//     diesel::insert_into(items::table)
//         .values(item)
//         .get_result::<Item>(&mut conn)
// }
//
// pub async fn create_new_character_item(state: SharedState, wz_id: i32) -> QueryResult<Item> {
//     let db = {
//         let state = state.lock().await;
//         state.db.clone()
//     };
//     let mut conn = db.get().map_err(|e| {
//         diesel::result::Error::DatabaseError(
//             diesel::result::DatabaseErrorKind::UnableToSendCommand,
//             Box::new(e.to_string()),
//         )
//     })?;
//     let item_img = String::from(wz_id);
//     item_img.push(".img");
//     let reader = wz::service::load_wz_item_reader(state.clone()).await?;
//     let root = reader.read_root_dir();
//     let etc = reader.read_dir_node(root.get("Etc").unwrap().as_dir().unwrap())?;
//     let img_hdr = etc.get(item_img).unwrap().as_img().unwrap();
//     let mut img = reader.img_reader(img_hdr)?;
//     diesel::insert_into(items::table)
//         .values(item)
//         .get_result::<Item>(&mut conn)
// }
