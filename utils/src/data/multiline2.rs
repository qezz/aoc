use std::{collections::HashMap, hash::Hash, str::FromStr};

use super::multiline::KeyValue;

pub trait SingleLineParse<T> {
    type Err;
    fn parse_line(line: &str) -> Result<T, Self::Err>;
    fn merge(&mut self, other: T);
}

#[derive(Clone, Debug)]
struct SampleSet<K: Hash, V> {
    inner: HashMap<K, V>
}

// impl<K, V> PartialEq for SampleSet<K, V> {
//     fn eq(&self, other: &Self) -> bool {
//         // self.inner == other.inner
//         if self.inner.len() != other.inner.len() {
//             return false;
//         }

//         for (key, value) in self.inner {
//             let entry = other.inner.entry(key);
//             match entry {
//                 Entry::Occupied(_) => todo!(),
//                 Entry::Vacant(_) => todo!(),
//             }
//         }

//         true
//     }
// }

impl FromStr for KeyValue<String, String> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((first, second)) = s.split_once(':') {
            return Ok(KeyValue(first.into(), second.into()));
        }

        Err(())
    }
}

impl<K: Hash, V> Default for SampleSet<K, V> {
    fn default() -> Self {
        Self {
            inner: Default::default()
        }
    }
}

impl<K, V> SingleLineParse<HashMap<K, V>> for SampleSet<K, V>
where KeyValue<K, V>: FromStr,
      K: Eq + Hash
{
    type Err = ();

    fn parse_line(line: &str) -> Result<HashMap<K, V>, Self::Err> {
        let kvs: Result<HashMap<K, V>, _> =
            line.split(' ')
            .map(|sub| {
                sub.parse::<KeyValue<K, V>>()
                    .map(|kv| {
                        (kv.0, kv.1)
                    })
            })
            .collect();

        kvs.map_err(|_| ())
    }

    fn merge(&mut self, other: HashMap<K, V>) {
        self.inner.extend(other);
    }
}

pub struct MLP {
    sep: String,
}

impl MLP {
    pub fn new(sep: &str) -> Self {
        Self {
            sep: sep.into(),
        }
    }

    pub fn parse<O, T: Default + SingleLineParse<O>>(&self, input: &str) -> Vec<T> {
        let groups: Vec<_> = input.lines()
            .collect::<Vec<_>>();

        let groups: Vec<_> = groups.split(|&substr| substr == self.sep)
            .collect();
        let mut res = vec![];

        for group in groups {
            let mut d = T::default();
            for line in group {
                let maybe_part = T::parse_line(line);
                if let Ok(part) = maybe_part {
                    d.merge(part);
                }
            }

            res.push(d);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn woot() {
        let input = "abc:aaa
def:bbb

ghi:ccc
klm:ddd";
        let mlp = MLP::new("");
        let v: Vec<SampleSet<String, String>> = mlp.parse(input);

        assert_eq!(v, vec![
            SampleSet {
                inner: vec![("abc", "aaa"),
                            ("def", "bbb")]
                    .iter()
                    .map(|(x, y)| (x.to_string(), y.to_string()))
                    .collect::<HashMap<_, _>>(),
            }
        ]);
    }
}
