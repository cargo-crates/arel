use clap::{App, Arg};
use arel_cli::generators;

fn main() {
    let matches = App::new("arel")
        .about("Arel CLI")
        .version("0.0.1")
        .subcommand(
            App::new("generate")
                .about("Generate A Migration ile")
                .subcommand(
                App::new("model")
                        .arg(clap::arg!(<Model> "Model Name"))
                        .arg(clap::arg!([Defines]).multiple_occurrences(true).last(true))
                )
                .subcommand(
                    App::new("migration")
                        .about("generate a migration file")
                        .arg(clap::arg!(<Migration> "file name"))
                        .arg(clap::arg!([Defines]).multiple_occurrences(true).last(true))
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("model", sub_matches)) => {
                    let model = sub_matches.value_of("Model").expect("Model required");
                    let args = sub_matches.values_of("Defines").map(|vals| vals.collect::<Vec<_>>()).unwrap_or_default();
                    generators::generate_model(&model, &args);
                }
                Some(("migration", sub_matches)) => {
                    let migration = sub_matches.value_of("Migration").expect("Migration required");
                    let args = sub_matches.values_of("Defines").map(|vals| vals.collect::<Vec<_>>()).unwrap_or_default();
                    generators::generate_migration(&migration, &args);
                }
                _ => unreachable!()
            }
        },
        _ => unreachable!()
    }
}
