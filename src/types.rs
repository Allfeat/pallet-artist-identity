use super::*;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Default, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Metadata<BoundedString, Description, Url, Username> {
    pub(super) alias: BoundedString,
    pub(super) bio: Description,
    pub(super) profile_pic: Url,
    // TODO music styles

    // Socials
    pub(super) twitter: Username,
    pub(super) facebook: Url,
    pub(super) instagram: Username,

    // Music links
    pub(super) spotify: Url,
    pub(super) apple_music: Url,

    // Dev
    pub(super) test: BoundedString,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Field<BoundedString, Description, Url, Username> {
    Alias(BoundedString),
    Bio(Description),
    ProfilePic(Url),
    Twitter(Username),
    Facebook(Url),
    Instagram(Username),
    Spotify(Url),
    AppleMusic(Url),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldName {
    Alias,
    Bio,
    ProfilePic,
    Twitter,
    Facebook,
    Instagram,
    Spotify,
    AppleMusic,
}

pub type FieldType<T> = Field<
    BoundedVec<u8, <T as Config>::MaxDefaultStringLength>,
    BoundedVec<u8, <T as Config>::MaxDescriptionLength>,
    BoundedVec<u8, <T as Config>::MaxDefaultStringLength>,
    BoundedVec<u8, <T as Config>::MaxDefaultStringLength>,
>;
