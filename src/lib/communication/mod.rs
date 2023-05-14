//! Common infrastructure to pass messages between `swd` and `swctl`.

use self::{reply_specifics::SpecificAnswer, client::SpecificArgs};

pub mod client;
pub mod details;
pub mod reply_specifics;
pub mod request_specifics;
pub mod server;

// No convenient macro because `https://github.com/rust-lang/rust/issues/86935`

/// Map [`SpecificArgs`] to its corresponding [`SpecificAnswer`] with default
/// values.
pub fn args_to_default_ans(args: &SpecificArgs) -> SpecificAnswer {
    use SpecificArgs as A;
    use SpecificAnswer as B;
    match args {
        A::Info(_) => B::Info(Default::default()),
        A::Start(_) => B::Start(Default::default()),
        A::Stop(_) => B::Stop(Default::default()),
        A::Play(_) => B::Play(Default::default()),
        A::Pause(_) => B::Pause(Default::default()),
        A::Lap(_) => B::Lap(Default::default()),
        A::Delete(_) => B::Delete(Default::default()),
    }
}

/// Map [`SpecificAnswer`] to its corresponding [`SpecificArgs`] with default
/// values.
pub fn ans_to_default_args(ans: &SpecificAnswer) -> SpecificArgs {
    use SpecificArgs as B;
    use SpecificAnswer as A;
    match ans {
        A::Info(_) => B::Info(Default::default()),
        A::Start(_) => B::Start(Default::default()),
        A::Stop(_) => B::Stop(Default::default()),
        A::Play(_) => B::Play(Default::default()),
        A::Pause(_) => B::Pause(Default::default()),
        A::Lap(_) => B::Lap(Default::default()),
        A::Delete(_) => B::Delete(Default::default()),
    }
}