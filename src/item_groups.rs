pub fn get_group(s: &str) -> Vec<&str> {
    match s {
        "Update 2 Powerup" => vec![
            "SMB2 Mushroom",
            "Frog Suit",
            "Power Balloon",
            "Super Acorn",
            "Boomerang Flower",
        ],
        _ => s.split([',', '/'].as_slice()).collect(),
    }
}
