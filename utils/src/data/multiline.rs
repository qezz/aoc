use std::{
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData
};

pub enum Sep {
    EmptyLine,
}

pub struct KeyValue<K, V>(K, V);

pub struct Invalid;

pub trait FromKVs {
    fn from_kvs(hm: &HashMap<String, String>) -> Result<Self, Invalid>
        where Self: Sized;
}

pub struct MultilineParser<T> {
    sep: Sep,
    phantom: PhantomData<T>,
}

impl<T> MultilineParser<T> {
    pub fn new(sep: Sep) -> Self {
        Self {
            sep,
            phantom: PhantomData,
        }
    }
}
impl<T: Debug> MultilineParser<T> {
    pub fn group_rows(&self, input: &str) -> Vec<Vec<String>> {
        let mut res:  Vec<Vec<String>> = vec![];
        let mut current = vec![];

        for line in input.lines() {
            if line.is_empty() {
                res.push(current.clone());
                current.clear();
            } else {
                current.push(line.to_string());
            }
        }

        if !current.is_empty() {
            res.push(current);
        }

        res
    }
}

impl<T: FromKVs + Debug> MultilineParser<T> {
    pub fn parse(&self, input: &str) -> Vec<T> {
        let mut res = vec![];
        let mut current = HashMap::new();
        for line in input.lines() {

            if line.is_empty() {
                // println!("> {:#?}", current);
                let pass = T::from_kvs(&current);
                if let Ok(p) = pass {
                    res.push(p);
                }
                current.clear();
            } else {
                let _sp: Vec<_> =
                    line
                    .split(' ')
                    .map(|x| {
                        let kv: Vec<String> = x.split(':').map(|y| y.to_string()).collect();
                        current.insert(kv[0].clone(), kv[1].clone());
                    })
                    .collect();
            }
        }

        if !current.is_empty() {
            let pass = T::from_kvs(&current);
            if let Ok(p) = pass {
                res.push(p);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn woot() {
//         let input = "abc:aaa
// def:bbb

// ghi:ccc
// klm:ddd";
//         let mlp: MultilineParser<Passport> = MultilineParser::new(Sep::EmptyLine);
//         mlp.parse(input);
//     }
}
