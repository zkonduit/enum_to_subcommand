pub use tosubcommand_derive::ToSubcommand;


pub trait ToSubcommand {
    /// Convert the struct to a subcommand string
    fn to_subcommand(&self) -> Vec<String>;
}
