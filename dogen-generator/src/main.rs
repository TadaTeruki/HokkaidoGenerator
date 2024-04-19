use name_engine::{Name, NameGeneratorBuilder};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let csv_file = include_str!("../dataset/placenames.csv");

    let place_names = csv_file
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let split = line.split(',');
            let phrases = split
                .last()
                .unwrap()
                .split(':')
                .map(|phrase| {
                    let mut split = phrase.split('_');
                    (split.next().unwrap(), split.next().unwrap())
                })
                .collect::<Vec<(&str, &str)>>();
            if let Ok(name) = Name::new(phrases) {
                Some(name)
            } else {
                None
            }
        })
        .collect::<Vec<Name>>();

    let generator = NameGeneratorBuilder::new()
        .bulk_add_names(place_names)
        .build();
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    (0..100).for_each(|_| {
        let name = generator.generate(|| rng.gen());
        println!("{} {}", name.0, name.1);
    });
}
