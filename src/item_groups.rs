pub fn get_group(s: &str) -> Vec<&str> {
    match s {
        "Style Powerup 1" => vec![
            "Big Mushroom",
            "Super Leaf",
            "Cape Feather",
            "Propeller Mushroom",
            "Super Bell",
        ],
        "Style Powerup 2" | "Update 2 Powerup" => vec![
            "SMB2 Mushroom",
            "Frog Suit",
            "Power Balloon",
            "Super Acorn",
            "Boomerang Flower",
        ],
        _ => s.split([',', '/'].as_slice()).collect(),
    }
}
