
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BagType {
    pub color: String,
}

impl BagType {
    pub fn colored(s: &str) -> Self {
        Self {
            color: s.into(),
        }
    }
}



#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BagD {
  pub typ: BagType,
  pub contains: Vec<(usize, BagType)>,
}
