use config::Config;

mod config_types;

fn main() {
    // load config
    let settings = Config::builder()
        .add_source(config::File::with_name("config.json"))
        .add_source(config::Environment::with_prefix("TRAILS").separator("_"))
        .build()
        .unwrap();

    println!("{:#?}", settings.try_deserialize::<config_types::Config>())

}
