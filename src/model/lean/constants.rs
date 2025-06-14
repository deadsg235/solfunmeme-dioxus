//use super::level::Level;
//use crate::model::lean::LevelType;
use crate::level::LevelType;
pub const LEVEL_U1: LevelType = LevelType::Param("u_1");
pub const LEVEL_U2: LevelType = LevelType::Param("u_2");
pub const LEVEL_U3: LevelType = LevelType::Param("u_3");
pub const LEVEL_U4: LevelType = LevelType::Param("u_4");
pub const LEVEL_U5: LevelType = LevelType::Param("u_5");
pub const LEVEL_U6: LevelType = LevelType::Param("u_6");
pub const LEVEL_U7: LevelType = LevelType::Param("u_7");
pub const LEVEL_U8: LevelType = LevelType::Param("u_8");

pub fn levels_8() -> Vec<LevelType<'static>> {
    vec![
        LEVEL_U1,
        LEVEL_U2,
        LEVEL_U3,
        LEVEL_U4,
        LEVEL_U5,
        LEVEL_U6,
        LEVEL_U7,
        LEVEL_U8,
    ]
}