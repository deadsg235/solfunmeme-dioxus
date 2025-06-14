#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_emoji_forall_const_sort() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
                        "type": "forallE",
                        "forbndrTypB": null,
                        "forbndrTyp": null,
                        "forbdB": null,
                        "forbd": null,
                        "binderName": "x",
                        "binderInfo": "default"
                    }
                },
                "name": "foo",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": null
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        let emoji = json_to_emoji(&json_str).unwrap();
        assert!(emoji.contains("∀ x (default:"));
    }

    #[test]
    fn test_json_to_emoji_const_with_levels() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
                        "type": "const",
                        "levels": [
                            { "level": "u", "kind": "param" },
                            { "level": "v", "kind": "param" }
                        ],
                        "declName": "bar"
                    }
                },
                "name": "bar",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": null
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        #[test]
        fn test_json_to_emoji_forall_with_nested_lam() {
            let json = json!({
                "kind": "AsyncConstB",
                "cnstInfB": {
                    "sig": {
                        "type": {
                            "type": "forallE",
                            "forbndrTypB": null,
                            "forbndrTyp": null,
                            "forbdB": {
                                "type": "lam",
                                "lambndrTpB": null,
                                "lambndrTp": null,
                                "lambdB": null,
                                "lambd": null,
                                "binderName": "z",
                                "binderInfo": "implicit"
                            },
                            "forbd": null,
                            "binderName": "x",
                            "binderInfo": "default"
                        }
                    },
                    "name": "nested_forall_lam",
                    "levelParams": [],
                    "kind": { "value": "def", "kind": "constant" },
                    "cnstInf": null
                }
            });
            let json_str = serde_json::to_string(&json).unwrap();
            let emoji = json_to_emoji(&json_str).unwrap();
            assert!(emoji.contains("∀ x (default:"));
            assert!(emoji.contains("λ z (implicit:"));
        }

        #[test]
        fn test_json_to_emoji_const_no_levels() {
            let json = json!({
                "kind": "AsyncConstB",
                "cnstInfB": {
                    "sig": {
                        "type": {
                            "type": "const",
                            "levels": [],
                            "declName": "no_levels"
                        }
                    },
                    "name": "no_levels",
                    "levelParams": [],
                    "kind": { "value": "def", "kind": "constant" },
                    "cnstInf": null
                }
            });
            let json_str = serde_json::to_string(&json).unwrap();
            let emoji = json_to_emoji(&json_str).unwrap();
            assert!(emoji.contains("🔖 no_levels []"));
        }

        #[test]
        fn test_json_to_emoji_lam_with_type_and_body() {
            let json = json!({
                "kind": "AsyncConstB",
                "cnstInfB": {
                    "sig": {
                        "type": {
                            "type": "lam",
                            "lambndrTpB": {
                                "type": "sort",
                                "level": { "level": "l", "kind": "param" }
                            },
                            "lambndrTp": null,
                            "lambdB": {
                                "type": "bvar"
                            },
                            "lambd": null,
                            "binderName": "a",
                            "binderInfo": "default"
                        }
                    },
                    "name": "lam_with_type",
                    "levelParams": [],
                    "kind": { "value": "def", "kind": "constant" },
                    "cnstInf": null
                }
            });
            let json_str = serde_json::to_string(&json).unwrap();
            let emoji = json_to_emoji(&json_str).unwrap();
            assert!(emoji.contains("λ a (default:"));
            assert!(emoji.contains("📏 l"));
            assert!(emoji.contains("📍"));
        }

        #[test]
        fn test_json_to_emoji_app_nested() {
            let json = json!({
                "kind": "AsyncConstB",
                "cnstInfB": {
                    "sig": {
                        "type": {
                            "type": "app",
                            "fn": {
                                "type": "app",
                                "fn": {
                                    "type": "bvar"
                                },
                                "arg": {
                                    "type": "bvar"
                                }
                            },
                            "arg": {
                                "type": "bvar"
                            }
                        }
                    },
                    "name": "app_nested",
                    "levelParams": [],
                    "kind": { "value": "def", "kind": "constant" },
                    "cnstInf": null
                }
            });
            let json_str = serde_json::to_string(&json).unwrap();
            let emoji = json_to_emoji(&json_str).unwrap();
            assert!(emoji.matches("➡️").count() >= 1);
            assert!(emoji.matches("📍").count() >= 3);
        }

        #[test]
        fn test_json_to_emoji_with_multiple_rules() {
            let json = json!({
                "kind": "AsyncConstB",
                "cnstInfB": {
                    "sig": {
                        "type": {
                            "type": "sort",
                            "level": { "level": "l", "kind": "param" }
                        }
                    },
                    "name": "multi_rules",
                    "levelParams": [],
                    "kind": { "value": "def", "kind": "constant" },
                    "cnstInf": {
                        "type": { "type": "sort", "level": { "level": "l", "kind": "param" } },
                        "numParams": 2,
                        "numMotives": 1,
                        "numMinors": 0,
                        "numIndices": 0,
                        "name": "multi_rules",
                        "levelParams": [],
                        "kind": "def",
                        "k": false,
                        "isUnsafe": false,
                        "all": [],
                        "Rules": [
                            {
                                "rhs": { "type": "bvar" },
                                "nfields": 1,
                                "name": "ruleA",
                                "kind": "rule"
                            },
                            {
                                "rhs": { "type": "sort", "level": { "level": "m", "kind": "param" } },
                                "nfields": 3,
                                "name": "ruleB",
                                "kind": "rule"
                            }
                        ]
                    }
                }
            });
            let json_str = serde_json::to_string(&json).unwrap();
            let emoji = json_to_emoji(&json_str).unwrap();
            assert!(emoji.contains("📜 Rules:"));
            assert!(emoji.contains("📋 ruleA (fields: 1)"));
            assert!(emoji.contains("📋 ruleB (fields: 3)"));
            assert!(emoji.contains("📏 m"));
        }
        assert!(emoji.contains("🔖 bar [u,v]"));
    }

    #[test]
    fn test_json_to_emoji_sort() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
                        "type": "sort",
                        "level": { "level": "l", "kind": "param" }
                    }
                },
                "name": "baz",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": null
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        let emoji = json_to_emoji(&json_str).unwrap();
        assert!(emoji.contains("📏 l"));
    }

    #[test]
    fn test_json_to_emoji_app_lam() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
                        "type": "app",
                        "fn": {
                            "type": "lam",
                            "lambndrTpB": null,
                            "lambndrTp": null,
                            "lambdB": null,
                            "lambd": null,
                            "binderName": "y",
                            "binderInfo": "default"
                        },
                        "arg": {
                            "type": "bvar"
                        }
                    }
                },
                "name": "qux",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": null
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        let emoji = json_to_emoji(&json_str).unwrap();
        assert!(emoji.contains("➡️"));
        assert!(emoji.contains("λ y (default:"));
        assert!(emoji.contains("📍"));
    }

    #[test]
    fn test_json_to_emoji_with_rules() {
        let json = json!({
            "kind": "AsyncConstB",
            "cnstInfB": {
                "sig": {
                    "type": {
                        "type": "sort",
                        "level": { "level": "l", "kind": "param" }
                    }
                },
                "name": "with_rules",
                "levelParams": [],
                "kind": { "value": "def", "kind": "constant" },
                "cnstInf": {
                    "type": { "type": "sort", "level": { "level": "l", "kind": "param" } },
                    "numParams": 1,
                    "numMotives": 1,
                    "numMinors": 0,
                    "numIndices": 0,
                    "name": "with_rules",
                    "levelParams": [],
                    "kind": "def",
                    "k": false,
                    "isUnsafe": false,
                    "all": [],
                    "Rules": [
                        {
                            "rhs": {
                                "type": "bvar"
                            },
                            "nfields": 2,
                            "name": "rule1",
                            "kind": "rule"
                        }
                    ]
                }
            }
        });
        let json_str = serde_json::to_string(&json).unwrap();
        let emoji = json_to_emoji(&json_str).unwrap();
        assert!(emoji.contains("📜 Rules:"));
        assert!(emoji.contains("📋 rule1 (fields: 2)"));
        assert!(emoji.contains("📍"));
    }
}
