struct RawRecord {
    name: String,
    quantity: String,
    rarity: String,
}

#[derive(Debug)]
struct CheckedItem {
    name: String,
    quantity: u32,
    rarity: Rarity,
}

#[derive(Debug)]
enum Rarity {
    Common,
    Rare,
}

impl Rarity {
    fn parse(rarity: &str) -> Option<Rarity> {
        match rarity {
            "Common" => Some(Rarity::Common),
            "Rare" => Some(Rarity::Rare),
            _ => None,
        }
    }
}

enum Outcome {
    Valid(CheckedItem),
    EmptyName,
    InvalidQuantity,
    UnknownRarity,
}

fn none_if_empty(name: &str) -> Option<&str> {
    if name.trim().is_empty() {
        None
    } else {
        Some(name.trim())
    }
}

fn check_record(record: &RawRecord) -> Outcome {
    let Some(name) = none_if_empty(&record.name) else {
        return Outcome::EmptyName;
    };
    let Ok(quantity @ 1..) = record.quantity.parse::<u32>() else {
        return Outcome::InvalidQuantity;
    };
    let Some(rarity) = Rarity::parse(&record.rarity) else {
        return Outcome::UnknownRarity;
    };
    let item = CheckedItem {
        name: String::from(name),
        quantity,
        rarity,
    };
    Outcome::Valid(item)
}

fn main() {
    let records = [
        RawRecord {
            name: String::new(),
            quantity: String::from("20"),
            rarity: String::from("Common"),
        },
        RawRecord {
            name: String::from("    "),
            quantity: String::from("20"),
            rarity: String::from("Common"),
        },
        RawRecord {
            name: String::from("    Test    "),
            quantity: String::from("j"),
            rarity: String::from("Common"),
        },
        RawRecord {
            name: String::from("Test"),
            quantity: String::from("-1"),
            rarity: String::from("Common"),
        },
        RawRecord {
            name: String::from("Test"),
            quantity: String::from("20"),
            rarity: String::from("Uncommon"),
        },
        RawRecord {
            name: String::from("Test"),
            quantity: String::from("0"),
            rarity: String::from("Rare"),
        },
        RawRecord {
            name: String::from("Test"),
            quantity: String::from("20"),
            rarity: String::from("Rare"),
        },
        RawRecord {
            name: String::from("Test"),
            quantity: String::from("20"),
            rarity: String::from("Common"),
        },
    ];
    let mut missing_name = 0;
    let mut invalid_quantity = 0;
    let mut unknown_rarity = 0;
    let mut valid = 0;
    for record in &records {
        match check_record(record) {
            Outcome::EmptyName => {
                missing_name += 1;
            }
            Outcome::InvalidQuantity => {
                invalid_quantity += 1;
            }
            Outcome::UnknownRarity => {
                unknown_rarity += 1;
            }
            Outcome::Valid(item) => {
                println!("{:#?}", item);
                valid += 1;
            }
        }
    }
    println!("Missing item name: {}", missing_name);
    println!("Item quantity is invalid: {}", invalid_quantity);
    println!("Unrecognized rarity: {}", unknown_rarity);
    println!("Valid items: {}", valid);
}
