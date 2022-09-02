use clap::Parser;

mod install;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long = "as", value_enum, default_value = "binary")]
    install_as: InstallAs,
    #[clap(long = "target", value_parser)]
    install_target: String
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum InstallAs {
    Service,
    Binary
}

fn main() {
    let cli = Cli::parse();

    println!("as: {:?}", cli.install_as);
    println!("target: {:?}", cli.install_target);
}
