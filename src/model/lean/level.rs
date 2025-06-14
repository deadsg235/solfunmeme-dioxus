//use std::collections::HashMap;



#[derive(Debug, Clone, PartialEq)]
pub enum LevelType<'a> {
    Zero,
    //Succ(Box<LevelType>),
    //Max(Box<LevelType>, Box<LevelType>),
    //IMax(Box<LevelType>, Box<LevelType>),
    Param(&'a str),
    MVar(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LevelDescr {
    level: String,
    kind: String,
}
