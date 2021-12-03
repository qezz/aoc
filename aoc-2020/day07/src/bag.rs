
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BagType {
    color: String,
}

impl BagType {
    pub fn colored(s: &str) -> Self {
        Self {
            color: s.into(),
        }
    }
}

pub struct BagDefinition {
    typ: BagType,
    contains: Vec<BagType>,
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BagD {
  pub typ: BagType,
  pub contains: Vec<BagType>,
}
