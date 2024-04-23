use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<()> {
    let mut password = Vec::new();
    let mut rng = rand::thread_rng();
    let mut chars = Vec::new();
    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER wont be empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER wont be empty"));
    }
    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).expect("NUMBERS wont be empty"));
    }
    if symbols {
        chars.extend_from_slice(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).expect("SYMBOLS wont be empty"));
    }
    password.shuffle(&mut rng);
    for _ in 0..length - password.len() as u8 {
        //let idx = rng.gen_range(0..chars.len());
        let c = chars
            .choose(&mut rng)
            .expect("chars wont be empty in this case");
        password.push(*c);
    }
    let password = String::from_utf8(password)?;
    println!("{}", password);

    let estimate = zxcvbn(&password, &[])?;
    eprintln!("Estimated strength: {}", estimate.score());
    Ok(())
}
