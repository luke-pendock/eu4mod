#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct Identifier {
    pub name: String,
//    pub value_type: Type,
}

impl Identifier {
    pub fn as_string(&self) -> String {
        format!(
            "{}", 
            self.name, 
        //    self.value_type.as_string()
        )
    }
}