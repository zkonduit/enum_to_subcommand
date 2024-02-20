use std::fmt::Debug;
pub use tosubcommand_derive::ToFlags;
pub use tosubcommand_derive::ToSubcommand;

pub trait ToSubcommand {
    /// Convert the enum elements to a subcommand string
    fn to_subcommand(&self) -> Vec<String>;
}

pub trait ToFlags: Debug {
    /// Convert the struct to a subcommand string
    fn to_flags(&self) -> Vec<String> {
        vec![format!("{:?}", self)]
    }

    /// is flag
    fn is_flag(&self) -> bool {
        false
    }

    /// is value
    fn is_value(&self) -> bool {
        true
    }

    /// is optional
    fn is_optional(&self) -> bool {
        false
    }
}

impl ToFlags for String {
    /// Convert the struct to a subcommand string
    fn to_flags(&self) -> Vec<String> {
        vec![format!("{}", self)]
    }
}
impl ToFlags for i128 {}
impl ToFlags for i64 {}
impl ToFlags for i32 {}
impl ToFlags for i16 {}
impl ToFlags for i8 {}
impl ToFlags for u128 {}
impl ToFlags for u64 {}
impl ToFlags for u32 {}
impl ToFlags for u16 {}
impl ToFlags for u8 {}
impl ToFlags for usize {}
impl ToFlags for f64 {}
impl ToFlags for f32 {}
impl ToFlags for bool {}
impl ToFlags for char {}

impl<T: ToFlags> ToFlags for Vec<T> {
    /// Convert the struct to a subcommand string
    fn to_flags(&self) -> Vec<String> {
        vec![self
            .iter()
            .map(|x| x.to_flags())
            .flatten()
            .collect::<Vec<String>>()
            .join(",")]
    }
}

impl<T: ToFlags, G: ToFlags> ToFlags for (T, G) {
    /// Convert the struct to a subcommand string
    fn to_flags(&self) -> Vec<String> {
        // write it as T->G
        vec![format!(
            "{}->{}",
            self.0.to_flags().join(""),
            self.1.to_flags().join("")
        )]
    }
}

impl ToFlags for std::path::PathBuf {
    /// Convert the struct to a subcommand string
    fn to_flags(&self) -> Vec<String> {
        vec![format!("{}", self.display())]
    }
}

impl<T: ToFlags> ToFlags for Option<T> {
    /// Convert the struct to a subcommand string
    fn to_flags(&self) -> Vec<String> {
        if let Some(val) = self {
            val.to_flags()
        } else {
            vec![]
        }
    }

    /// is optional
    fn is_optional(&self) -> bool {
        true
    }
}
