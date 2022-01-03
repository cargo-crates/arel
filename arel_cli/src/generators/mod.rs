use std::{env, fs};

pub fn make_migration_dir() -> String {
    let path = env::current_dir().unwrap();
    let migration_dir = format!("{}/migrations", path.to_str().unwrap());
    fs::create_dir_all(&migration_dir).unwrap();
    migration_dir
}

pub fn generate_model(model: &str, args: &Vec<&str>) {
    let migration_dir = make_migration_dir();
    let file_prefix = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    let table_name = inflector::string::pluralize::to_plural(&inflector::cases::snakecase::to_snake_case(model));
    let full_path_file = format!("{}/{}_create_{}", migration_dir, file_prefix, table_name);
    fs::File::create(&full_path_file).expect("create failed");


    println!("{}, {:?}", model, args);
}

pub fn generate_migration(migration: &str, args: &Vec<&str>) {
    let migration_dir = make_migration_dir();
    let file_prefix = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    let table_name = regex::Regex::new(r"to_(\w{1,})$").unwrap().captures(migration).unwrap().get(0).unwrap().as_str().to_string().replace("to_", "");
    let full_path_file = format!("{}/{}_{}", migration_dir, file_prefix, migration);
    fs::File::create(&full_path_file).expect("create failed");

    println!("table_name, {}", table_name);


    println!("{}, {:?}", migration, args);
}