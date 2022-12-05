pub fn get_ponies() -> Vec<&'static str> {
    vec![
        "applejack_hat_large",
        "applejack_large",
        "celestia_large",
        "fluttershy_large",
        #[cfg(target_os = "linux")]
        "fluttershy",
        "luna_large",
        "mcintosh_large",
        "pinkiepie_large",
        "rainbowdash_large",
        #[cfg(target_os = "linux")]
        "rainbowdash",
        #[cfg(target_os = "linux")]
        "rarity",
        "twilight_large",
        #[cfg(target_os = "linux")]
        "twilight",
    ]
}