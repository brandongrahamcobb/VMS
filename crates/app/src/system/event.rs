use bevy::ecs::message::Message;
use ipc::event::AsyncEvent;

#[derive(Message)]
pub enum RawEvent {
    // General
    PacketReceived(AsyncEvent),
    ClientTransitioning(AsyncEvent),
    ClientConnected(AsyncEvent),
    ClientDisconnected(AsyncEvent),
    // Login
    LoginValid(AsyncEvent),
    LoginInvalid(AsyncEvent),
    CharCreationSuccess(AsyncEvent),
    ListCharsSuccess(AsyncEvent),
    ListCharsFailed(AsyncEvent),
    CheckCharName(AsyncEvent),
    SelectCharWithPic(AsyncEvent),
    // Game
    JoinSuccess(AsyncEvent),
    PickupSuccess(AsyncEvent),
    CloseAttackSuccess(AsyncEvent),
    ChangeMapSuccess(AsyncEvent),
    DeadMobSuccess(AsyncEvent),
}
