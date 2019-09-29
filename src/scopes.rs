use std::{
    cmp::{Ordering, PartialEq, PartialOrd},
    collections::HashSet,
    fmt,
    ops::BitOr,
    str::FromStr,
};

use serde::ser::{Serialize, Serializer};

use errors::Error;
use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor};

/// Represents a set of OAuth scopes
///
/// # Example
///
/// ```rust
/// use elefren::prelude::*;
///
/// let read = Scopes::read_all();
/// let write = Scopes::write_all();
/// let follow = Scopes::follow();
/// let all = read | write | follow;
/// ```
#[derive(Clone)]
pub struct Scopes {
    scopes: HashSet<Scope>,
}

impl FromStr for Scopes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Scopes, Self::Err> {
        let mut set = HashSet::new();
        for scope in s.split_whitespace() {
            let scope = Scope::from_str(&scope)?;
            set.insert(scope);
        }
        Ok(Scopes {
            scopes: set,
        })
    }
}

impl Serialize for Scopes {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let repr = format!("{}", self);
        serializer.serialize_str(&repr)
    }
}

struct DeserializeScopesVisitor;

impl<'de> Visitor<'de> for DeserializeScopesVisitor {
    type Value = Scopes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "space separated scopes")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        Scopes::from_str(v).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Scopes {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_str(DeserializeScopesVisitor)
    }
}

impl Scopes {
    /// Represents all available oauth scopes: "read write follow push"
    ///
    /// ```
    /// # extern crate elefren;
    /// # use std::error::Error;
    /// use elefren::scopes::Scopes;
    ///
    /// # fn main() -> Result<(), Box<Error>> {
    /// let scope = Scopes::all();
    /// assert_eq!(&format!("{}", scope), "read write follow push");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn all() -> Scopes {
        Scopes::read_all() | Scopes::write_all() | Scopes::follow() | Scopes::push()
    }

    /// Represents the full "read" scope
    ///
    /// ```
    /// # extern crate elefren;
    /// # use std::error::Error;
    /// use elefren::scopes::Scopes;
    ///
    /// # fn main() -> Result<(), Box<Error>> {
    /// let scope = Scopes::read_all();
    /// assert_eq!(&format!("{}", scope), "read");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn read_all() -> Scopes {
        Scopes::_read(None)
    }

    /// Represents a specific "read:___" scope
    ///
    /// ```
    /// # extern crate elefren;
    /// # use std::error::Error;
    /// use elefren::scopes::{Read, Scopes};
    ///
    /// # fn main() -> Result<(), Box<Error>> {
    /// let scope = Scopes::read(Read::Accounts);
    /// assert_eq!(&format!("{}", scope), "read:accounts");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn read(subscope: Read) -> Scopes {
        Scopes::_read(Some(subscope))
    }

    /// Represents the full "write" scope
    ///
    /// ```
    /// # extern crate elefren;
    /// # use std::error::Error;
    /// use elefren::scopes::Scopes;
    ///
    /// # fn main() -> Result<(), Box<Error>> {
    /// let scope = Scopes::write_all();
    /// assert_eq!(&format!("{}", scope), "write");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn write_all() -> Scopes {
        Scopes::_write(None)
    }

    /// Represents a specific "write:___" scope
    ///
    /// ```
    /// # extern crate elefren;
    /// # use std::error::Error;
    /// use elefren::scopes::{Scopes, Write};
    ///
    /// # fn main() -> Result<(), Box<Error>> {
    /// let scope = Scopes::write(Write::Accounts);
    /// assert_eq!(&format!("{}", scope), "write:accounts");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn write(subscope: Write) -> Scopes {
        Scopes::_write(Some(subscope))
    }

    /// Represents the "follow" scope
    ///
    /// ```
    /// # extern crate elefren;
    /// # use std::error::Error;
    /// use elefren::scopes::Scopes;
    ///
    /// # fn main() -> Result<(), Box<Error>> {
    /// let scope = Scopes::follow();
    /// assert_eq!(&format!("{}", scope), "follow");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn follow() -> Scopes {
        Scopes::new(Scope::Follow)
    }

    /// Represents the full "push" scope
    ///
    /// ```
    /// # extern crate elefren;
    /// # use std::error::Error;
    /// use elefren::scopes::Scopes;
    ///
    /// # fn main() -> Result<(), Box<Error>> {
    /// let scope = Scopes::push();
    /// assert_eq!(&format!("{}", scope), "push");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn push() -> Scopes {
        Scopes::new(Scope::Push)
    }

    /// Combines 2 scopes together
    ///
    /// # Example
    ///
    /// ```rust
    /// use elefren::prelude::*;
    ///
    /// let read = Scopes::read_all();
    /// let write = Scopes::write_all();
    /// let read_write = read.and(write);
    /// ```
    pub fn and(self, other: Scopes) -> Scopes {
        let newset: HashSet<_> = self
            .scopes
            .union(&other.scopes)
            .into_iter()
            .map(|s| *s)
            .collect();
        Scopes {
            scopes: newset,
        }
    }

    fn _write(subscope: Option<Write>) -> Scopes {
        Scopes::new(Scope::Write(subscope))
    }

    fn _read(subscope: Option<Read>) -> Scopes {
        Scopes::new(Scope::Read(subscope))
    }

    fn new(scope: Scope) -> Scopes {
        let mut set = HashSet::new();
        set.insert(scope);
        Scopes {
            scopes: set,
        }
    }
}

impl BitOr for Scopes {
    type Output = Scopes;

    fn bitor(self, other: Scopes) -> Self::Output {
        self.and(other)
    }
}

impl PartialEq for Scopes {
    fn eq(&self, other: &Scopes) -> bool {
        self.scopes
            .symmetric_difference(&other.scopes)
            .next()
            .is_none()
    }
}

impl Default for Scopes {
    fn default() -> Scopes {
        Scopes::read_all()
    }
}

impl fmt::Debug for Scopes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for scope in &self.scopes {
            write!(f, "{:?}", &scope)?;
        }
        Ok(write!(f, "]")?)
    }
}

impl fmt::Display for Scopes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut start = true;
        let scopes = {
            let mut scopes = self.scopes.iter().collect::<Vec<_>>();
            scopes.sort();
            scopes
        };
        for scope in &scopes {
            if !start {
                write!(f, " ")?;
            } else {
                start = false;
            }
            write!(f, "{}", &scope)?;
        }
        Ok(())
    }
}

/// Permission scope of the application.
/// [Details on what each permission provides][1]
/// [1]: https://github.com/tootsuite/documentation/blob/master/Using-the-API/OAuth-details.md)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Serialize)]
enum Scope {
    /// Read only permissions.
    #[serde(rename = "read")]
    Read(Option<Read>),
    /// Write only permissions.
    #[serde(rename = "write")]
    Write(Option<Write>),
    /// Only permission to add and remove followers.
    #[serde(rename = "follow")]
    Follow,
    /// Push permissions
    #[serde(rename = "push")]
    Push,
}

impl FromStr for Scope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Scope, Self::Err> {
        Ok(match s {
            "read" => Scope::Read(None),
            "write" => Scope::Write(None),
            "follow" => Scope::Follow,
            "push" => Scope::Push,
            read if read.starts_with("read:") => {
                let r: Read = Read::from_str(&read[5..])?;
                Scope::Read(Some(r))
            },
            write if write.starts_with("write:") => {
                let w: Write = Write::from_str(&write[6..])?;
                Scope::Write(Some(w))
            },
            _ => return Err(Error::Other("Unknown scope".to_string())),
        })
    }
}

impl Ord for Scope {
    fn cmp(&self, other: &Scope) -> Ordering {
        match (*self, *other) {
            (Scope::Read(None), Scope::Read(None)) => Ordering::Equal,
            (Scope::Read(None), Scope::Read(Some(..))) => Ordering::Less,
            (Scope::Read(Some(..)), Scope::Read(None)) => Ordering::Greater,
            (Scope::Read(Some(ref a)), Scope::Read(Some(ref b))) => a.cmp(b),

            (Scope::Write(None), Scope::Write(None)) => Ordering::Equal,
            (Scope::Write(None), Scope::Write(Some(..))) => Ordering::Less,
            (Scope::Write(Some(..)), Scope::Write(None)) => Ordering::Greater,
            (Scope::Write(Some(ref a)), Scope::Write(Some(ref b))) => a.cmp(b),

            (Scope::Read(..), Scope::Write(..)) => Ordering::Less,
            (Scope::Read(..), Scope::Follow) => Ordering::Less,
            (Scope::Read(..), Scope::Push) => Ordering::Less,

            (Scope::Write(..), Scope::Read(..)) => Ordering::Greater,
            (Scope::Write(..), Scope::Follow) => Ordering::Less,
            (Scope::Write(..), Scope::Push) => Ordering::Less,

            (Scope::Follow, Scope::Read(..)) => Ordering::Greater,
            (Scope::Follow, Scope::Write(..)) => Ordering::Greater,
            (Scope::Follow, Scope::Follow) => Ordering::Equal,
            (Scope::Follow, Scope::Push) => Ordering::Less,

            (Scope::Push, Scope::Push) => Ordering::Equal,
            (Scope::Push, _) => Ordering::Greater,
        }
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Scope::*;
        let s = match *self {
            Read(Some(ref r)) => return fmt::Display::fmt(r, f),
            Read(None) => "read",
            Write(Some(ref w)) => return fmt::Display::fmt(w, f),
            Write(None) => "write",
            Follow => "follow",
            Push => "push",
        };
        write!(f, "{}", s)
    }
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Read(None)
    }
}

/// Represents the granular "read:___" oauth scopes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Read {
    /// Accounts
    #[serde(rename = "accounts")]
    Accounts,
    /// Blocks
    #[serde(rename = "blocks")]
    Blocks,
    /// Favourites
    #[serde(rename = "favourites")]
    Favourites,
    /// Filters
    #[serde(rename = "filters")]
    Filters,
    /// Follows
    #[serde(rename = "follows")]
    Follows,
    /// Lists
    #[serde(rename = "lists")]
    Lists,
    /// Mutes
    #[serde(rename = "mutes")]
    Mutes,
    /// Notifications
    #[serde(rename = "notifications")]
    Notifications,
    /// Reports
    #[serde(rename = "reports")]
    Reports,
    /// Search
    #[serde(rename = "search")]
    Search,
    /// Statuses
    #[serde(rename = "statuses")]
    Statuses,
}

impl FromStr for Read {
    type Err = Error;

    fn from_str(s: &str) -> Result<Read, Self::Err> {
        Ok(match s {
            "accounts" => Read::Accounts,
            "blocks" => Read::Blocks,
            "favourites" => Read::Favourites,
            "filters" => Read::Filters,
            "follows" => Read::Follows,
            "lists" => Read::Lists,
            "mutes" => Read::Mutes,
            "notifications" => Read::Notifications,
            "reports" => Read::Reports,
            "search" => Read::Search,
            "statuses" => Read::Statuses,
            _ => return Err(Error::Other("Unknown 'read' subcategory".to_string())),
        })
    }
}

impl PartialOrd for Read {
    fn partial_cmp(&self, other: &Read) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Read {
    fn cmp(&self, other: &Read) -> Ordering {
        let a = format!("{:?}", self);
        let b = format!("{:?}", other);
        a.cmp(&b)
    }
}

impl fmt::Display for Read {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "read:{}",
            match *self {
                Read::Accounts => "accounts",
                Read::Blocks => "blocks",
                Read::Favourites => "favourites",
                Read::Filters => "filters",
                Read::Follows => "follows",
                Read::Lists => "lists",
                Read::Mutes => "mutes",
                Read::Notifications => "notifications",
                Read::Reports => "reports",
                Read::Search => "search",
                Read::Statuses => "statuses",
            }
        )
    }
}

/// Represents the granular "write:___" oauth scopes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Write {
    /// Accounts
    #[serde(rename = "accounts")]
    Accounts,
    /// Blocks
    #[serde(rename = "blocks")]
    Blocks,
    /// Favourites
    #[serde(rename = "favourites")]
    Favourites,
    /// Filters
    #[serde(rename = "filters")]
    Filters,
    /// Follows
    #[serde(rename = "follows")]
    Follows,
    /// Lists
    #[serde(rename = "lists")]
    Lists,
    /// Media
    #[serde(rename = "media")]
    Media,
    /// Mutes
    #[serde(rename = "mutes")]
    Mutes,
    /// Notifications
    #[serde(rename = "notifications")]
    Notifications,
    /// Reports
    #[serde(rename = "reports")]
    Reports,
    /// Statuses
    #[serde(rename = "statuses")]
    Statuses,
}

impl FromStr for Write {
    type Err = Error;

    fn from_str(s: &str) -> Result<Write, Self::Err> {
        Ok(match s {
            "accounts" => Write::Accounts,
            "blocks" => Write::Blocks,
            "favourites" => Write::Favourites,
            "filters" => Write::Filters,
            "follows" => Write::Follows,
            "lists" => Write::Lists,
            "media" => Write::Media,
            "mutes" => Write::Mutes,
            "notifications" => Write::Notifications,
            "reports" => Write::Reports,
            "statuses" => Write::Statuses,
            _ => return Err(Error::Other("Unknown 'write' subcategory".to_string())),
        })
    }
}

impl PartialOrd for Write {
    fn partial_cmp(&self, other: &Write) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Write {
    fn cmp(&self, other: &Write) -> Ordering {
        let a = format!("{:?}", self);
        let b = format!("{:?}", other);
        a.cmp(&b)
    }
}

impl fmt::Display for Write {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "write:{}",
            match *self {
                Write::Accounts => "accounts",
                Write::Blocks => "blocks",
                Write::Favourites => "favourites",
                Write::Filters => "filters",
                Write::Follows => "follows",
                Write::Lists => "lists",
                Write::Media => "media",
                Write::Mutes => "mutes",
                Write::Notifications => "notifications",
                Write::Reports => "reports",
                Write::Statuses => "statuses",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_write_cmp() {
        let tests = [
            (Write::Accounts, Write::Blocks),
            (Write::Blocks, Write::Favourites),
            (Write::Favourites, Write::Filters),
            (Write::Filters, Write::Follows),
            (Write::Follows, Write::Lists),
            (Write::Lists, Write::Media),
            (Write::Media, Write::Mutes),
            (Write::Mutes, Write::Notifications),
            (Write::Notifications, Write::Reports),
            (Write::Reports, Write::Statuses),
        ];

        for (a, b) in &tests {
            assert!(a < b);
            assert!(b > a);
        }
    }

    #[test]
    fn test_read_cmp() {
        let tests = [
            (Read::Accounts, Read::Blocks),
            (Read::Blocks, Read::Favourites),
            (Read::Favourites, Read::Filters),
            (Read::Filters, Read::Follows),
            (Read::Follows, Read::Lists),
            (Read::Lists, Read::Mutes),
            (Read::Mutes, Read::Notifications),
            (Read::Notifications, Read::Reports),
            (Read::Reports, Read::Search),
            (Read::Search, Read::Statuses),
        ];
        for (a, b) in &tests {
            assert!(a < b);
            assert!(b > a);
        }
    }

    #[test]
    fn test_scope_cmp() {
        let tests = [
            (Scope::Read(None), Scope::Read(Some(Read::Accounts))),
            (Scope::Read(None), Scope::Read(Some(Read::Blocks))),
            (Scope::Read(None), Scope::Read(Some(Read::Favourites))),
            (Scope::Read(None), Scope::Read(Some(Read::Filters))),
            (Scope::Read(None), Scope::Read(Some(Read::Follows))),
            (Scope::Read(None), Scope::Read(Some(Read::Lists))),
            (Scope::Read(None), Scope::Read(Some(Read::Mutes))),
            (Scope::Read(None), Scope::Read(Some(Read::Notifications))),
            (Scope::Read(None), Scope::Read(Some(Read::Reports))),
            (Scope::Read(None), Scope::Read(Some(Read::Search))),
            (Scope::Read(None), Scope::Read(Some(Read::Statuses))),
            (Scope::Read(Some(Read::Statuses)), Scope::Write(None)),
            (Scope::Read(Some(Read::Mutes)), Scope::Follow),
            (Scope::Read(None), Scope::Push),
            (Scope::Write(None), Scope::Write(Some(Write::Accounts))),
            (Scope::Write(None), Scope::Write(Some(Write::Blocks))),
            (Scope::Write(None), Scope::Write(Some(Write::Favourites))),
            (Scope::Write(None), Scope::Write(Some(Write::Filters))),
            (Scope::Write(None), Scope::Write(Some(Write::Follows))),
            (Scope::Write(None), Scope::Write(Some(Write::Lists))),
            (Scope::Write(None), Scope::Write(Some(Write::Media))),
            (Scope::Write(None), Scope::Write(Some(Write::Mutes))),
            (Scope::Write(None), Scope::Write(Some(Write::Notifications))),
            (Scope::Write(None), Scope::Write(Some(Write::Reports))),
            (Scope::Write(None), Scope::Write(Some(Write::Statuses))),
            (Scope::Write(Some(Write::Statuses)), Scope::Follow),
            (Scope::Write(Some(Write::Follows)), Scope::Push),
        ];

        for (a, b) in &tests {
            assert!(a < b);
        }
    }

    #[test]
    fn test_scope_display() {
        let values = [
            Scope::Read(None),
            Scope::Read(Some(Read::Accounts)),
            Scope::Read(Some(Read::Blocks)),
            Scope::Read(Some(Read::Favourites)),
            Scope::Read(Some(Read::Filters)),
            Scope::Read(Some(Read::Follows)),
            Scope::Read(Some(Read::Lists)),
            Scope::Read(Some(Read::Mutes)),
            Scope::Read(Some(Read::Notifications)),
            Scope::Read(Some(Read::Reports)),
            Scope::Read(Some(Read::Search)),
            Scope::Read(Some(Read::Statuses)),
            Scope::Write(None),
            Scope::Write(Some(Write::Accounts)),
            Scope::Write(Some(Write::Blocks)),
            Scope::Write(Some(Write::Favourites)),
            Scope::Write(Some(Write::Filters)),
            Scope::Write(Some(Write::Follows)),
            Scope::Write(Some(Write::Lists)),
            Scope::Write(Some(Write::Media)),
            Scope::Write(Some(Write::Mutes)),
            Scope::Write(Some(Write::Notifications)),
            Scope::Write(Some(Write::Reports)),
            Scope::Write(Some(Write::Statuses)),
            Scope::Follow,
            Scope::Push,
        ];

        let expecteds = [
            "read".to_string(),
            "read:accounts".to_string(),
            "read:blocks".to_string(),
            "read:favourites".to_string(),
            "read:filters".to_string(),
            "read:follows".to_string(),
            "read:lists".to_string(),
            "read:mutes".to_string(),
            "read:notifications".to_string(),
            "read:reports".to_string(),
            "read:search".to_string(),
            "read:statuses".to_string(),
            "write".to_string(),
            "write:accounts".to_string(),
            "write:blocks".to_string(),
            "write:favourites".to_string(),
            "write:filters".to_string(),
            "write:follows".to_string(),
            "write:lists".to_string(),
            "write:media".to_string(),
            "write:mutes".to_string(),
            "write:notifications".to_string(),
            "write:reports".to_string(),
            "write:statuses".to_string(),
            "follow".to_string(),
            "push".to_string(),
        ];

        let tests = values.into_iter().zip(expecteds.into_iter());

        for (value, expected) in tests {
            let result = value.to_string();
            assert_eq!(&result, expected);
        }
    }

    #[test]
    fn test_scopes_default() {
        let default: Scope = Default::default();
        assert_eq!(default, Scope::Read(None));
    }

    #[test]
    fn test_scopes_display() {
        let tests = [
            (
                Scopes::read(Read::Accounts) | Scopes::follow(),
                "read:accounts follow",
            ),
            (
                Scopes::read(Read::Follows) | Scopes::read(Read::Accounts) | Scopes::write_all(),
                "read:accounts read:follows write",
            ),
        ];

        for (a, b) in &tests {
            assert_eq!(&format!("{}", a), b);
        }
    }

    #[test]
    fn test_scopes_serialize_deserialize() {
        let tests = [
            (
                Scopes::read_all() | Scopes::write(Write::Notifications) | Scopes::follow(),
                "read write:notifications follow",
            ),
            (Scopes::follow() | Scopes::push(), "follow push"),
        ];

        for (a, b) in &tests {
            let ser = serde_json::to_string(&a).expect("Couldn't serialize Scopes");
            let expected = format!("\"{}\"", b);
            assert_eq!(&ser, &expected);

            let des : Scopes = serde_json::from_str(&ser).expect("Couldn't deserialize Scopes");
            assert_eq!(&des, a);
        }
    }

    #[test]
    fn test_scope_from_str() {
        let tests = [
            ("read", Scope::Read(None)),
            ("read:accounts", Scope::Read(Some(Read::Accounts))),
            ("read:blocks", Scope::Read(Some(Read::Blocks))),
            ("read:favourites", Scope::Read(Some(Read::Favourites))),
            ("read:filters", Scope::Read(Some(Read::Filters))),
            ("read:follows", Scope::Read(Some(Read::Follows))),
            ("read:lists", Scope::Read(Some(Read::Lists))),
            ("read:mutes", Scope::Read(Some(Read::Mutes))),
            ("read:notifications", Scope::Read(Some(Read::Notifications))),
            ("read:reports", Scope::Read(Some(Read::Reports))),
            ("read:search", Scope::Read(Some(Read::Search))),
            ("read:statuses", Scope::Read(Some(Read::Statuses))),
            ("write", Scope::Write(None)),
            ("write:accounts", Scope::Write(Some(Write::Accounts))),
            ("write:blocks", Scope::Write(Some(Write::Blocks))),
            ("write:favourites", Scope::Write(Some(Write::Favourites))),
            ("write:filters", Scope::Write(Some(Write::Filters))),
            ("write:follows", Scope::Write(Some(Write::Follows))),
            ("write:lists", Scope::Write(Some(Write::Lists))),
            ("write:media", Scope::Write(Some(Write::Media))),
            ("write:mutes", Scope::Write(Some(Write::Mutes))),
            (
                "write:notifications",
                Scope::Write(Some(Write::Notifications)),
            ),
            ("write:reports", Scope::Write(Some(Write::Reports))),
            ("write:statuses", Scope::Write(Some(Write::Statuses))),
            ("follow", Scope::Follow),
            ("push", Scope::Push),
        ];
        for (source, expected) in &tests {
            let result = Scope::from_str(source).expect(&format!("Couldn't parse '{}'", &source));
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn test_scopes_str_round_trip() {
        let original = "read write follow push";
        let scopes = Scopes::from_str(original).expect("Couldn't convert to Scopes");
        let result = format!("{}", scopes);
        assert_eq!(original, result);
    }
}
