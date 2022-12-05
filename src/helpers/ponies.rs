use std::collections::HashMap;
pub struct Pony {
    pub text: String,
    pub lines: u16
}

pub fn get_pony(name: String) -> Option<Pony> {
    let ponies: HashMap<&str, Vec<u8>> = HashMap::from([
        ("applejack_hat_large", include_bytes!("../../ponies/applejack_hat_large.txt").to_vec()),
        ("applejack_hat_large", include_bytes!("../../ponies/applejack_large.txt").to_vec()),
        ("celestia_large", include_bytes!("../../ponies/celestia_large.txt").to_vec()),
        ("fluttershy_large", include_bytes!("../../ponies/fluttershy_large.txt").to_vec()),
        ("fluttershy", include_bytes!("../../ponies/fluttershy.txt").to_vec()),
        ("luna_large", include_bytes!("../../ponies/luna_large.txt").to_vec()),
        ("mcintosh_large", include_bytes!("../../ponies/mcintosh_large.txt").to_vec()),
        ("pinkiepie_large", include_bytes!("../../ponies/pinkiepie_large.txt").to_vec()),
        ("rainbowdash_large", include_bytes!("../../ponies/rainbowdash_large.txt").to_vec()),
        ("rainbowdash", include_bytes!("../../ponies/rainbowdash.txt").to_vec()),
        ("rarity", include_bytes!("../../ponies/rarity.txt").to_vec()),
        ("twilight_large", include_bytes!("../../ponies/twilight_large.txt").to_vec()),
        ("twilight", include_bytes!("../../ponies/twilight.txt").to_vec()),
    ]);

    let pony = String::from_utf8(
            ponies.get(
            name.as_str()
        ).unwrap().to_vec()
    ).unwrap();

    Some(Pony {
        text: pony.clone(),
        lines: pony.split("\n").count() as u16
    })
}