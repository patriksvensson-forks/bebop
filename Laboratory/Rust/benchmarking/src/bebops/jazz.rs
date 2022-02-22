//
// This code was generated by a tool.
//
//
//   bebopc version:
//       2.3.1
//
//
//   bebopc source:
//       https://github.com/RainwayApp/bebop
//
//
// Changes to this file may cause incorrect behavior and will be lost if
// the code is regenerated.
//

#![allow(warnings)]

use bebop::FixedSized as _;
use bebop::Record as _;
use core::convert::TryInto as _;
use std::io::Write as _;

pub type _DynFut<T> = ::core::pin::Pin<::std::boxed::Box<dyn::core::future::Future<Output = T>>>;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instrument {
    Sax = 0,
    Trumpet = 1,
    Clarinet = 2,
    Piano = 3,
    Cello = 4,
}

impl ::core::convert::TryFrom<u32> for Instrument {
    type Error = ::bebop::DeserializeError;

    fn try_from(value: u32) -> ::bebop::DeResult<Self> {
        match value {
            0 => Ok(Instrument::Sax),
            1 => Ok(Instrument::Trumpet),
            2 => Ok(Instrument::Clarinet),
            3 => Ok(Instrument::Piano),
            4 => Ok(Instrument::Cello),
            d => Err(::bebop::DeserializeError::InvalidEnumDiscriminator(
                d.into(),
            )),
        }
    }
}

impl ::core::convert::From<Instrument> for u32 {
    fn from(value: Instrument) -> Self {
        match value {
            Instrument::Sax => 0,
            Instrument::Trumpet => 1,
            Instrument::Clarinet => 2,
            Instrument::Piano => 3,
            Instrument::Cello => 4,
        }
    }
}

impl ::bebop::SubRecord<'_> for Instrument {
    const MIN_SERIALIZED_SIZE: usize = ::std::mem::size_of::<u32>();
    const EXACT_SERIALIZED_SIZE: Option<usize> = Some(::std::mem::size_of::<u32>());

    #[inline]
    fn serialized_size(&self) -> usize {
        ::std::mem::size_of::<u32>()
    }

    #[inline]
    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        u32::from(*self)._serialize_chained(dest)
    }

    #[inline]
    fn _deserialize_chained(raw: &[u8]) -> ::bebop::DeResult<(usize, Self)> {
        let (n, v) = u32::_deserialize_chained(raw)?;
        Ok((n, v.try_into()?))
    }
}

impl ::bebop::FixedSized for Instrument {
    const SERIALIZED_SIZE: usize = ::std::mem::size_of::<u32>();
}

#[derive(Clone, Debug, PartialEq)]
pub struct Performer<'raw> {
    pub name: &'raw str,
    pub plays: Instrument,
}

impl<'raw> ::bebop::SubRecord<'raw> for Performer<'raw> {
    const MIN_SERIALIZED_SIZE: usize =
        <&'raw str>::MIN_SERIALIZED_SIZE + <Instrument>::MIN_SERIALIZED_SIZE;

    #[inline]
    fn serialized_size(&self) -> usize {
        self.name.serialized_size() + self.plays.serialized_size()
    }

    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        Ok(self.name._serialize_chained(dest)? + self.plays._serialize_chained(dest)?)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let mut i = 0;
        if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
            let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
            return Err(::bebop::DeserializeError::MoreDataExpected(missing));
        }

        let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;

        Ok((
            i,
            Self {
                name: v0,
                plays: v1,
            },
        ))
    }
}

impl<'raw> ::bebop::Record<'raw> for Performer<'raw> {}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Song<'raw> {
    /// Field 1
    pub title: ::core::option::Option<&'raw str>,
    /// Field 2
    pub year: ::core::option::Option<u16>,
    /// Field 3
    pub performers: ::core::option::Option<::std::vec::Vec<Performer<'raw>>>,
}

impl<'raw> ::bebop::SubRecord<'raw> for Song<'raw> {
    const MIN_SERIALIZED_SIZE: usize = ::bebop::LEN_SIZE + 1;

    #[inline]
    fn serialized_size(&self) -> usize {
        ::bebop::LEN_SIZE
            + 1
            + self
                .title
                .as_ref()
                .map(|v| v.serialized_size() + 1)
                .unwrap_or(0)
            + self
                .year
                .as_ref()
                .map(|v| v.serialized_size() + 1)
                .unwrap_or(0)
            + self
                .performers
                .as_ref()
                .map(|v| v.serialized_size() + 1)
                .unwrap_or(0)
    }

    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        let size = self.serialized_size();
        ::bebop::write_len(dest, size - ::bebop::LEN_SIZE)?;
        if let Some(ref v) = self.title {
            1u8._serialize_chained(dest)?;
            v._serialize_chained(dest)?;
        }
        if let Some(ref v) = self.year {
            2u8._serialize_chained(dest)?;
            v._serialize_chained(dest)?;
        }
        if let Some(ref v) = self.performers {
            3u8._serialize_chained(dest)?;
            v._serialize_chained(dest)?;
        }
        0u8._serialize_chained(dest)?;
        Ok(size)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let mut i = 0;
        let len = ::bebop::read_len(&raw[i..])? + ::bebop::LEN_SIZE;
        i += ::bebop::LEN_SIZE;

        #[cfg(not(feature = "bebop-unchecked"))]
        if len == 0 {
            return Err(::bebop::DeserializeError::CorruptFrame);
        }

        if raw.len() < len {
            return Err(::bebop::DeserializeError::MoreDataExpected(len - raw.len()));
        }

        let mut _title = None;
        let mut _year = None;
        let mut _performers = None;

        #[cfg(not(feature = "bebop-unchecked"))]
        let mut last = 0;

        while i < len {
            let di = raw[i];

            #[cfg(not(feature = "bebop-unchecked"))]
            if di != 0 {
                if di < last {
                    return Err(::bebop::DeserializeError::CorruptFrame);
                }
                last = di;
            }

            i += 1;
            match di {
                0 => {
                    break;
                }
                1 => {
                    #[cfg(not(feature = "bebop-unchecked"))]
                    if _title.is_some() {
                        return Err(::bebop::DeserializeError::DuplicateMessageField);
                    }
                    let (read, value) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;
                    _title = Some(value)
                }
                2 => {
                    #[cfg(not(feature = "bebop-unchecked"))]
                    if _year.is_some() {
                        return Err(::bebop::DeserializeError::DuplicateMessageField);
                    }
                    let (read, value) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;
                    _year = Some(value)
                }
                3 => {
                    #[cfg(not(feature = "bebop-unchecked"))]
                    if _performers.is_some() {
                        return Err(::bebop::DeserializeError::DuplicateMessageField);
                    }
                    let (read, value) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;
                    _performers = Some(value)
                }
                _ => {
                    i = len;
                    break;
                }
            }
        }

        if i != len {
            debug_assert!(i > len);
            return Err(::bebop::DeserializeError::CorruptFrame);
        }

        Ok((
            i,
            Self {
                title: _title,
                year: _year,
                performers: _performers,
            },
        ))
    }
}

impl<'raw> ::bebop::Record<'raw> for Song<'raw> {}

#[derive(Clone, Debug, PartialEq)]
pub enum Album<'raw> {
    /// An unknown type which is likely defined in a newer version of the schema.
    Unknown,

    /// Discriminator 1
    StudioAlbum { tracks: ::std::vec::Vec<Song<'raw>> },

    /// Discriminator 2
    LiveAlbum {
        /// Field 1
        tracks: ::core::option::Option<::std::vec::Vec<Song<'raw>>>,
        /// Field 2
        venue_name: ::core::option::Option<&'raw str>,
        /// Field 3
        concert_date: ::core::option::Option<::bebop::Date>,
    },
}

impl<'raw> ::bebop::SubRecord<'raw> for Album<'raw> {
    const MIN_SERIALIZED_SIZE: usize = ::bebop::LEN_SIZE + 1;

    fn serialized_size(&self) -> usize {
        ::bebop::LEN_SIZE
            + 1
            + match self {
                Album::Unknown => 0,
                Self::StudioAlbum {
                    tracks: ref _tracks,
                } => _tracks.serialized_size(),
                Self::LiveAlbum {
                    tracks: ref _tracks,
                    venue_name: ref _venue_name,
                    concert_date: ref _concert_date,
                } => {
                    ::bebop::LEN_SIZE
                        + 1
                        + _tracks
                            .as_ref()
                            .map(|v| v.serialized_size() + 1)
                            .unwrap_or(0)
                        + _venue_name
                            .as_ref()
                            .map(|v| v.serialized_size() + 1)
                            .unwrap_or(0)
                        + _concert_date
                            .as_ref()
                            .map(|v| v.serialized_size() + 1)
                            .unwrap_or(0)
                }
            }
    }

    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        let size = self.serialized_size();
        ::bebop::write_len(dest, size - ::bebop::LEN_SIZE - 1)?;
        match self {
            Album::Unknown => {
                return Err(::bebop::SerializeError::CannotSerializeUnknownUnion);
            }
            Self::StudioAlbum {
                tracks: ref _tracks,
            } => {
                1u8._serialize_chained(dest)?;
                _tracks._serialize_chained(dest)?;
            }
            Self::LiveAlbum {
                tracks: ref _tracks,
                venue_name: ref _venue_name,
                concert_date: ref _concert_date,
            } => {
                2u8._serialize_chained(dest)?;
                ::bebop::write_len(dest, size - ::bebop::LEN_SIZE * 2 - 1)?;
                if let Some(ref v) = _tracks {
                    1u8._serialize_chained(dest)?;
                    v._serialize_chained(dest)?;
                }
                if let Some(ref v) = _venue_name {
                    2u8._serialize_chained(dest)?;
                    v._serialize_chained(dest)?;
                }
                if let Some(ref v) = _concert_date {
                    3u8._serialize_chained(dest)?;
                    v._serialize_chained(dest)?;
                }
                0u8._serialize_chained(dest)?;
            }
        }
        Ok(size)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let len = ::bebop::read_len(&raw)? + ::bebop::LEN_SIZE + 1;
        let mut i = ::bebop::LEN_SIZE + 1;
        let de = match raw[::bebop::LEN_SIZE] {
            1 => {
                let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;

                Album::StudioAlbum { tracks: v0 }
            }
            2 => {
                let len = ::bebop::read_len(&raw[i..])? + i + ::bebop::LEN_SIZE;
                i += ::bebop::LEN_SIZE;

                #[cfg(not(feature = "bebop-unchecked"))]
                if len == 0 {
                    return Err(::bebop::DeserializeError::CorruptFrame);
                }

                if raw.len() < len {
                    return Err(::bebop::DeserializeError::MoreDataExpected(len - raw.len()));
                }

                let mut _tracks = None;
                let mut _venue_name = None;
                let mut _concert_date = None;

                #[cfg(not(feature = "bebop-unchecked"))]
                let mut last = 0;

                while i < len {
                    let di = raw[i];

                    #[cfg(not(feature = "bebop-unchecked"))]
                    if di != 0 {
                        if di < last {
                            return Err(::bebop::DeserializeError::CorruptFrame);
                        }
                        last = di;
                    }

                    i += 1;
                    match di {
                        0 => {
                            break;
                        }
                        1 => {
                            #[cfg(not(feature = "bebop-unchecked"))]
                            if _tracks.is_some() {
                                return Err(::bebop::DeserializeError::DuplicateMessageField);
                            }
                            let (read, value) =
                                ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                            i += read;
                            _tracks = Some(value)
                        }
                        2 => {
                            #[cfg(not(feature = "bebop-unchecked"))]
                            if _venue_name.is_some() {
                                return Err(::bebop::DeserializeError::DuplicateMessageField);
                            }
                            let (read, value) =
                                ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                            i += read;
                            _venue_name = Some(value)
                        }
                        3 => {
                            #[cfg(not(feature = "bebop-unchecked"))]
                            if _concert_date.is_some() {
                                return Err(::bebop::DeserializeError::DuplicateMessageField);
                            }
                            let (read, value) =
                                ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                            i += read;
                            _concert_date = Some(value)
                        }
                        _ => {
                            i = len;
                            break;
                        }
                    }
                }

                if i != len {
                    debug_assert!(i > len);
                    return Err(::bebop::DeserializeError::CorruptFrame);
                }

                Album::LiveAlbum {
                    tracks: _tracks,
                    venue_name: _venue_name,
                    concert_date: _concert_date,
                }
            }
            _ => {
                i = len;
                Album::Unknown
            }
        };
        if !cfg!(feature = "bebop-unchecked") && i != len {
            debug_assert!(i > len);
            Err(::bebop::DeserializeError::CorruptFrame)
        } else {
            Ok((i, de))
        }
    }
}

impl<'raw> ::bebop::Record<'raw> for Album<'raw> {}

#[derive(Clone, Debug, PartialEq)]
pub struct Library<'raw> {
    pub albums: ::std::collections::HashMap<&'raw str, Album<'raw>>,
}

impl<'raw> ::core::ops::Deref for Library<'raw> {
    type Target = ::std::collections::HashMap<&'raw str, Album<'raw>>;

    fn deref(&self) -> &Self::Target {
        &self.albums
    }
}

impl<'raw> ::core::ops::DerefMut for Library<'raw> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.albums
    }
}

impl<'raw> ::bebop::SubRecord<'raw> for Library<'raw> {
    const MIN_SERIALIZED_SIZE: usize =
        <::std::collections::HashMap<&'raw str, Album<'raw>>>::MIN_SERIALIZED_SIZE;

    #[inline]
    fn serialized_size(&self) -> usize {
        self.albums.serialized_size()
    }

    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        Ok(self.albums._serialize_chained(dest)?)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let mut i = 0;
        if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
            let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
            return Err(::bebop::DeserializeError::MoreDataExpected(missing));
        }

        let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;

        Ok((i, Self { albums: v0 }))
    }
}

impl<'raw> ::bebop::Record<'raw> for Library<'raw> {}

#[cfg(feature = "bebop-owned-all")]
pub mod owned {
    #![allow(warnings)]

    use bebop::FixedSized as _;
    use bebop::Record as _;
    use core::convert::TryInto as _;
    use std::io::Write as _;

    pub type _DynFut<T> =
        ::core::pin::Pin<::std::boxed::Box<dyn::core::future::Future<Output = T>>>;

    pub use super::Instrument;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Performer {
        pub name: ::std::string::String,
        pub plays: Instrument,
    }

    impl<'raw> ::core::convert::From<super::Performer<'raw>> for Performer {
        fn from(value: super::Performer) -> Self {
            Self {
                name: value.name.into(),
                plays: value.plays,
            }
        }
    }

    impl<'raw> ::bebop::SubRecord<'raw> for Performer {
        const MIN_SERIALIZED_SIZE: usize =
            <::std::string::String>::MIN_SERIALIZED_SIZE + <Instrument>::MIN_SERIALIZED_SIZE;

        #[inline]
        fn serialized_size(&self) -> usize {
            self.name.serialized_size() + self.plays.serialized_size()
        }

        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            Ok(self.name._serialize_chained(dest)? + self.plays._serialize_chained(dest)?)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let mut i = 0;
            if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
                let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
                return Err(::bebop::DeserializeError::MoreDataExpected(missing));
            }

            let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;
            let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;

            Ok((
                i,
                Self {
                    name: v0,
                    plays: v1,
                },
            ))
        }
    }

    impl<'raw> ::bebop::Record<'raw> for Performer {}

    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct Song {
        /// Field 1
        pub title: ::core::option::Option<::std::string::String>,
        /// Field 2
        pub year: ::core::option::Option<u16>,
        /// Field 3
        pub performers: ::core::option::Option<::std::vec::Vec<Performer>>,
    }

    impl<'raw> ::core::convert::From<super::Song<'raw>> for Song {
        fn from(value: super::Song) -> Self {
            Self {
                title: value.title.map(|value| value.into()),
                year: value.year,
                performers: value
                    .performers
                    .map(|value| value.into_iter().map(|value| value.into()).collect()),
            }
        }
    }

    impl<'raw> ::bebop::SubRecord<'raw> for Song {
        const MIN_SERIALIZED_SIZE: usize = ::bebop::LEN_SIZE + 1;

        #[inline]
        fn serialized_size(&self) -> usize {
            ::bebop::LEN_SIZE
                + 1
                + self
                    .title
                    .as_ref()
                    .map(|v| v.serialized_size() + 1)
                    .unwrap_or(0)
                + self
                    .year
                    .as_ref()
                    .map(|v| v.serialized_size() + 1)
                    .unwrap_or(0)
                + self
                    .performers
                    .as_ref()
                    .map(|v| v.serialized_size() + 1)
                    .unwrap_or(0)
        }

        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            let size = self.serialized_size();
            ::bebop::write_len(dest, size - ::bebop::LEN_SIZE)?;
            if let Some(ref v) = self.title {
                1u8._serialize_chained(dest)?;
                v._serialize_chained(dest)?;
            }
            if let Some(ref v) = self.year {
                2u8._serialize_chained(dest)?;
                v._serialize_chained(dest)?;
            }
            if let Some(ref v) = self.performers {
                3u8._serialize_chained(dest)?;
                v._serialize_chained(dest)?;
            }
            0u8._serialize_chained(dest)?;
            Ok(size)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let mut i = 0;
            let len = ::bebop::read_len(&raw[i..])? + ::bebop::LEN_SIZE;
            i += ::bebop::LEN_SIZE;

            #[cfg(not(feature = "bebop-unchecked"))]
            if len == 0 {
                return Err(::bebop::DeserializeError::CorruptFrame);
            }

            if raw.len() < len {
                return Err(::bebop::DeserializeError::MoreDataExpected(len - raw.len()));
            }

            let mut _title = None;
            let mut _year = None;
            let mut _performers = None;

            #[cfg(not(feature = "bebop-unchecked"))]
            let mut last = 0;

            while i < len {
                let di = raw[i];

                #[cfg(not(feature = "bebop-unchecked"))]
                if di != 0 {
                    if di < last {
                        return Err(::bebop::DeserializeError::CorruptFrame);
                    }
                    last = di;
                }

                i += 1;
                match di {
                    0 => {
                        break;
                    }
                    1 => {
                        #[cfg(not(feature = "bebop-unchecked"))]
                        if _title.is_some() {
                            return Err(::bebop::DeserializeError::DuplicateMessageField);
                        }
                        let (read, value) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                        i += read;
                        _title = Some(value)
                    }
                    2 => {
                        #[cfg(not(feature = "bebop-unchecked"))]
                        if _year.is_some() {
                            return Err(::bebop::DeserializeError::DuplicateMessageField);
                        }
                        let (read, value) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                        i += read;
                        _year = Some(value)
                    }
                    3 => {
                        #[cfg(not(feature = "bebop-unchecked"))]
                        if _performers.is_some() {
                            return Err(::bebop::DeserializeError::DuplicateMessageField);
                        }
                        let (read, value) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                        i += read;
                        _performers = Some(value)
                    }
                    _ => {
                        i = len;
                        break;
                    }
                }
            }

            if i != len {
                debug_assert!(i > len);
                return Err(::bebop::DeserializeError::CorruptFrame);
            }

            Ok((
                i,
                Self {
                    title: _title,
                    year: _year,
                    performers: _performers,
                },
            ))
        }
    }

    impl<'raw> ::bebop::Record<'raw> for Song {}

    #[derive(Clone, Debug, PartialEq)]
    pub enum Album {
        /// An unknown type which is likely defined in a newer version of the schema.
        Unknown,

        /// Discriminator 1
        StudioAlbum { tracks: ::std::vec::Vec<Song> },

        /// Discriminator 2
        LiveAlbum {
            /// Field 1
            tracks: ::core::option::Option<::std::vec::Vec<Song>>,
            /// Field 2
            venue_name: ::core::option::Option<::std::string::String>,
            /// Field 3
            concert_date: ::core::option::Option<::bebop::Date>,
        },
    }

    impl<'raw> ::core::convert::From<super::Album<'raw>> for Album {
        fn from(value: super::Album) -> Self {
            match value {
                super::Album::Unknown => Self::Unknown,
                super::Album::StudioAlbum { tracks: _tracks } => Self::StudioAlbum {
                    tracks: _tracks.into_iter().map(|value| value.into()).collect(),
                },
                super::Album::LiveAlbum {
                    tracks: _tracks,
                    venue_name: _venue_name,
                    concert_date: _concert_date,
                } => Self::LiveAlbum {
                    tracks: _tracks
                        .map(|value| value.into_iter().map(|value| value.into()).collect()),
                    venue_name: _venue_name.map(|value| value.into()),
                    concert_date: _concert_date,
                },
            }
        }
    }
    impl<'raw> ::bebop::SubRecord<'raw> for Album {
        const MIN_SERIALIZED_SIZE: usize = ::bebop::LEN_SIZE + 1;

        fn serialized_size(&self) -> usize {
            ::bebop::LEN_SIZE
                + 1
                + match self {
                    Album::Unknown => 0,
                    Self::StudioAlbum {
                        tracks: ref _tracks,
                    } => _tracks.serialized_size(),
                    Self::LiveAlbum {
                        tracks: ref _tracks,
                        venue_name: ref _venue_name,
                        concert_date: ref _concert_date,
                    } => {
                        ::bebop::LEN_SIZE
                            + 1
                            + _tracks
                                .as_ref()
                                .map(|v| v.serialized_size() + 1)
                                .unwrap_or(0)
                            + _venue_name
                                .as_ref()
                                .map(|v| v.serialized_size() + 1)
                                .unwrap_or(0)
                            + _concert_date
                                .as_ref()
                                .map(|v| v.serialized_size() + 1)
                                .unwrap_or(0)
                    }
                }
        }

        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            let size = self.serialized_size();
            ::bebop::write_len(dest, size - ::bebop::LEN_SIZE - 1)?;
            match self {
                Album::Unknown => {
                    return Err(::bebop::SerializeError::CannotSerializeUnknownUnion);
                }
                Self::StudioAlbum {
                    tracks: ref _tracks,
                } => {
                    1u8._serialize_chained(dest)?;
                    _tracks._serialize_chained(dest)?;
                }
                Self::LiveAlbum {
                    tracks: ref _tracks,
                    venue_name: ref _venue_name,
                    concert_date: ref _concert_date,
                } => {
                    2u8._serialize_chained(dest)?;
                    ::bebop::write_len(dest, size - ::bebop::LEN_SIZE * 2 - 1)?;
                    if let Some(ref v) = _tracks {
                        1u8._serialize_chained(dest)?;
                        v._serialize_chained(dest)?;
                    }
                    if let Some(ref v) = _venue_name {
                        2u8._serialize_chained(dest)?;
                        v._serialize_chained(dest)?;
                    }
                    if let Some(ref v) = _concert_date {
                        3u8._serialize_chained(dest)?;
                        v._serialize_chained(dest)?;
                    }
                    0u8._serialize_chained(dest)?;
                }
            }
            Ok(size)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let len = ::bebop::read_len(&raw)? + ::bebop::LEN_SIZE + 1;
            let mut i = ::bebop::LEN_SIZE + 1;
            let de = match raw[::bebop::LEN_SIZE] {
                1 => {
                    let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;

                    Album::StudioAlbum { tracks: v0 }
                }
                2 => {
                    let len = ::bebop::read_len(&raw[i..])? + i + ::bebop::LEN_SIZE;
                    i += ::bebop::LEN_SIZE;

                    #[cfg(not(feature = "bebop-unchecked"))]
                    if len == 0 {
                        return Err(::bebop::DeserializeError::CorruptFrame);
                    }

                    if raw.len() < len {
                        return Err(::bebop::DeserializeError::MoreDataExpected(len - raw.len()));
                    }

                    let mut _tracks = None;
                    let mut _venue_name = None;
                    let mut _concert_date = None;

                    #[cfg(not(feature = "bebop-unchecked"))]
                    let mut last = 0;

                    while i < len {
                        let di = raw[i];

                        #[cfg(not(feature = "bebop-unchecked"))]
                        if di != 0 {
                            if di < last {
                                return Err(::bebop::DeserializeError::CorruptFrame);
                            }
                            last = di;
                        }

                        i += 1;
                        match di {
                            0 => {
                                break;
                            }
                            1 => {
                                #[cfg(not(feature = "bebop-unchecked"))]
                                if _tracks.is_some() {
                                    return Err(::bebop::DeserializeError::DuplicateMessageField);
                                }
                                let (read, value) =
                                    ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                                i += read;
                                _tracks = Some(value)
                            }
                            2 => {
                                #[cfg(not(feature = "bebop-unchecked"))]
                                if _venue_name.is_some() {
                                    return Err(::bebop::DeserializeError::DuplicateMessageField);
                                }
                                let (read, value) =
                                    ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                                i += read;
                                _venue_name = Some(value)
                            }
                            3 => {
                                #[cfg(not(feature = "bebop-unchecked"))]
                                if _concert_date.is_some() {
                                    return Err(::bebop::DeserializeError::DuplicateMessageField);
                                }
                                let (read, value) =
                                    ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                                i += read;
                                _concert_date = Some(value)
                            }
                            _ => {
                                i = len;
                                break;
                            }
                        }
                    }

                    if i != len {
                        debug_assert!(i > len);
                        return Err(::bebop::DeserializeError::CorruptFrame);
                    }

                    Album::LiveAlbum {
                        tracks: _tracks,
                        venue_name: _venue_name,
                        concert_date: _concert_date,
                    }
                }
                _ => {
                    i = len;
                    Album::Unknown
                }
            };
            if !cfg!(feature = "bebop-unchecked") && i != len {
                debug_assert!(i > len);
                Err(::bebop::DeserializeError::CorruptFrame)
            } else {
                Ok((i, de))
            }
        }
    }

    impl<'raw> ::bebop::Record<'raw> for Album {}

    #[derive(Clone, Debug, PartialEq)]
    pub struct Library {
        pub albums: ::std::collections::HashMap<::std::string::String, Album>,
    }

    impl<'raw> ::core::convert::From<super::Library<'raw>> for Library {
        fn from(value: super::Library) -> Self {
            Self {
                albums: value
                    .albums
                    .into_iter()
                    .map(|(key, value)| (key.into(), value.into()))
                    .collect(),
            }
        }
    }

    impl ::core::ops::Deref for Library {
        type Target = ::std::collections::HashMap<::std::string::String, Album>;

        fn deref(&self) -> &Self::Target {
            &self.albums
        }
    }

    impl ::core::ops::DerefMut for Library {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.albums
        }
    }

    impl<'raw> ::bebop::SubRecord<'raw> for Library {
        const MIN_SERIALIZED_SIZE: usize =
            <::std::collections::HashMap<::std::string::String, Album>>::MIN_SERIALIZED_SIZE;

        #[inline]
        fn serialized_size(&self) -> usize {
            self.albums.serialized_size()
        }

        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            Ok(self.albums._serialize_chained(dest)?)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let mut i = 0;
            if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
                let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
                return Err(::bebop::DeserializeError::MoreDataExpected(missing));
            }

            let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;

            Ok((i, Self { albums: v0 }))
        }
    }

    impl<'raw> ::bebop::Record<'raw> for Library {}
}
