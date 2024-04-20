use name_engine::{Name, NameGeneratorBuilder, SyllableInfo};
use rand::{rngs::StdRng, Rng, SeedableRng};

pub struct NameConfig {
    pub target_name_length: f64,
    pub cmp_samples: usize,
}

pub struct NameGenerator {
    generator: name_engine::NameGenerator,
    rng: StdRng,
}

impl NameGenerator {
    pub fn new(dataset_source: &str, seed: usize) -> Self {
        let dataset = dataset_source
            .lines()
            .filter_map(|line| {
                if line.is_empty() {
                    return None;
                }
                let split = line.split(',');
                let phrases = split
                    .last()?
                    .split(':')
                    .filter_map(|phrase| {
                        let mut split = phrase.split('_');
                        Some((split.next()?, split.next()?))
                    })
                    .collect::<Vec<(&str, &str)>>();
                if let Ok(name) = Name::new(phrases) {
                    Some(name)
                } else {
                    None
                }
            })
            .collect::<Vec<Name>>();

        let generator = NameGeneratorBuilder::new().bulk_add_names(dataset).build();
        let rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
        Self { generator, rng }
    }

    fn evaluate(
        &self,
        name: &String,
        syllable_info: &[SyllableInfo],
        target_name_length: f64,
    ) -> Option<f64> {
        let first_syllable_place_name = self
            .generator
            .names()
            .get(syllable_info.first()?.name_index)?;
        if first_syllable_place_name.content() == *name {
            return None;
        }

        let has_duplicated_syllables = syllable_info
            .iter()
            .filter_map(|info: &SyllableInfo| {
                self.generator
                    .names()
                    .get(info.name_index)?
                    .syllables()
                    .get(info.syllable_index)
            })
            .fold((false, (String::new(), String::new())), |acc, syllable| {
                (
                    acc.0 || (syllable.0 == acc.1 .0) || (syllable.1 == acc.1 .1),
                    syllable.clone(),
                )
            })
            .0;

        if has_duplicated_syllables {
            return None;
        }
        Some(-(target_name_length - name.chars().count() as f64).abs())
    }

    pub fn generate(&mut self, config: NameConfig) -> Option<(String, String)> {
        (0..config.cmp_samples)
            .filter_map(|_| {
                let (name, pronunciation, syllable_info) =
                    self.generator.generate_verbose(|| self.rng.gen());
                let score = self.evaluate(&name, &syllable_info, config.target_name_length)?;
                Some((name, pronunciation, score))
            })
            .max_by(|(_, _, score1), (_, _, score2)| score1.total_cmp(score2))
            .map(|(name, pronunciation, _)| (name, pronunciation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_generator() {
        let csv_file = include_str!("../dataset/placenames.csv");
        let mut generator = NameGenerator::new(csv_file, 0);
        (0..1000).for_each(|_| {
            if let Some((name, pronunciation)) = generator.generate(NameConfig {
                target_name_length: 3.0,
                cmp_samples: 5,
            }) {
                println!("{}: {}", name, pronunciation);
            }
        });
    }
}
