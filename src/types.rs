use super::*;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type StyleListOf<T> = BoundedVec<MusicStyleName, <T as Config>::MaxRegisteredStyles>;
pub type MetadataOf<T> = Metadata<
    BoundedVec<u8, <T as Config>::MaxDefaultStringLength>,
    BoundedVec<u8, <T as Config>::MaxDescriptionLength>,
    BoundedVec<u8, <T as Config>::MaxDefaultStringLength>,
    BoundedVec<u8, <T as Config>::MaxDefaultStringLength>,
    StyleListOf<T>,
>;

#[derive(Clone, Default, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Metadata<BoundedString, Description, Url, Username, BoundedStyles> {
    pub(super) alias: BoundedString,
    pub(super) bio: Description,
    pub(super) profile_pic: Url,
    pub(super) music_styles: BoundedStyles,

    // Socials
    pub(super) twitter: Username,
    pub(super) facebook: Url,
    pub(super) instagram: Username,

    // Music links
    pub(super) spotify: Url,
    pub(super) apple_music: Url,
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

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
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
