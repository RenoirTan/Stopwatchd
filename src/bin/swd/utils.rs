//! Extra stuff.

use stopwatchd::communication::{
    client_message::ClientRequestKind,
    server_message::ServerReplyKind,
    start::StartReply,
    info::InfoReply,
    stop::StopReply,
    lap::LapReply,
    pause::PauseReply,
    play::PlayReply,
    delete::DeleteReply
};

/// Convert [`ClientRequestKind`] to corresponding [`ServerReplyKind`].
/// 
/// TODO: Integrated into stopwatchd library.
pub fn crk_to_srk(crk: &ClientRequestKind) -> ServerReplyKind {
    use ClientRequestKind as C;
    use ServerReplyKind as S;
    match crk {
        C::Start(_) => S::Start(StartReply),
        C::Info(_) => S::Info(InfoReply::default()),
        C::Stop(_) => S::Stop(StopReply),
        C::Lap(_) => S::Lap(LapReply),
        C::Pause(_) => S::Pause(PauseReply),
        C::Play(_) => S::Play(PlayReply),
        C::Delete(_) => S::Delete(DeleteReply),
        C::Default => S::Default
    }
}