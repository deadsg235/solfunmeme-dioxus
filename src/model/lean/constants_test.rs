use super::level::Level;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_constants() {
        assert_eq!(LEVEL_U1, Level::Param("u_1"));
        assert_eq!(LEVEL_U2, Level::Param("u_2"));
        assert_eq!(LEVEL_U3, Level::Param("u_3"));
        assert_eq!(LEVEL_U4, Level::Param("u_4"));
        assert_eq!(LEVEL_U5, Level::Param("u_5"));
        assert_eq!(LEVEL_U6, Level::Param("u_6"));
        assert_eq!(LEVEL_U7, Level::Param("u_7"));
        assert_eq!(LEVEL_U8, Level::Param("u_8"));
    }

    #[test]
    fn test_levels_8_contains_all_levels() {
        let levels = levels_8();
        assert_eq!(levels.len(), 8);
        assert_eq!(levels[0], LEVEL_U1);
        assert_eq!(levels[1], LEVEL_U2);
        assert_eq!(levels[2], LEVEL_U3);
        assert_eq!(levels[3], LEVEL_U4);
        assert_eq!(levels[4], LEVEL_U5);
        assert_eq!(levels[5], LEVEL_U6);
        assert_eq!(levels[6], LEVEL_U7);
        assert_eq!(levels[7], LEVEL_U8);
    }

    #[test]
    fn test_levels_8_unique() {
        let levels = levels_8();
        let mut unique = levels.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), 8);
    }
}