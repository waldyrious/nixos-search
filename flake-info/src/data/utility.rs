use serde::{Deserialize, Deserializer, Serialize, Serializer};


#[derive(Debug, Clone, PartialEq)]
pub struct Reverse<T: Reversable + Serialize>(pub T);

pub trait Reversable {
    fn reverse(&self) -> Self;
}

impl Reversable for String {
    fn reverse(&self) -> Self {
        self.chars().rev().collect::<String>()
    }
}

impl<T: Reversable + Clone> Reversable for Vec<T> {
    fn reverse(&self) -> Self {
        self.iter().cloned().map(|item| item.reverse()).collect()
    }
}

impl<T> Reversable for Reverse<T>
where
    T: Reversable + Serialize,
{
    fn reverse(&self) -> Self {
        Reverse(self.0.reverse())
    }
}

impl<T> Serialize for Reverse<T>
where
    T: Reversable + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.reverse().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Reverse<T>
where
    T: Reversable + Serialize + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Reverse<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Reverse(T::deserialize(deserializer)?.reverse()))
    }
}


/// A utility type that can represent the presence of either a single associated
/// value or a list of those. Adding absence can be achieved by wrapping the type
/// in an [Option]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    #[serde(serialize_with = "list")]
    One(T),
    Many(Vec<T>),
}

impl<T> OneOrMany<T> {
    pub fn into_list(self) -> Vec<T> {
        match self {
            OneOrMany::One(one) => vec![one],
            OneOrMany::Many(many) => many,
        }
    }
}

/// A utility type that flattens lists of lists as seen with `maintainers` and `platforms` on selected packages
/// in an [Option]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Flatten<T> {
    #[serde(serialize_with = "list")]
    Single(T),
    Deep(Vec<Flatten<T>>),
}

impl<T: Clone> Flatten<T> {
    pub fn flatten(self) -> Vec<T> {
        match self {
            Flatten::Single(s) => vec![s],
            Flatten::Deep(v) => v.into_iter().map(Flatten::flatten).flatten().collect(),
        }
    }
}

// TODO: use this or a to_ist function?
/// Serialization helper that serializes single elements as a list with a single
/// item
pub fn list<T, S>(item: &T, s: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    s.collect_seq(vec![item].iter())
}