// use crate::net::error::NetworkError;
// use crate::net::packet::core::Packet;
// use crate::net::packet::handler::action::world::WorldAction;
// use crate::net::packet::handler::result::HandlerResult;
// use crate::runtime::relay::RuntimeContext;
//
// pub struct LoginStartHandler;
//
// impl LoginStartHandler {
//     pub fn new() -> Self {
//         Self
//     }
//
//     pub async fn handle(
//         self: &Self,
//         _packet: Packet,
//     ) -> Result<HandlerResult<WorldAction>, NetworkError> {
//         let mut result = HandlerResult::new();
//         let action = WorldAction::Simple;
//         result.add_action(action)?;
//         Ok(result)
//     }
// }
