use super::*;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Default, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Metadatas<BoundedString, Description, URL, Username> {
    pub(super) alias: Option<BoundedString>,
    pub(super) bio: Option<Description>,
    pub(super) profile_pic: Option<URL>,
    // TODO music styles

    // Socials
    pub(super) twitter: Option<Username>,
    pub(super) fb: Option<URL>,
    pub(super) instagram: Option<Username>,

    // Music links
    pub(super) spotify: Option<URL>,
    pub(super) apple_music: Option<URL>,
}