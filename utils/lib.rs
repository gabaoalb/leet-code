pub mod listnode;
pub mod tree_node;
pub mod string_utils {
    pub fn remove_non_alphanumeric(s: &str) -> String {
        s.chars().filter(|c| c.is_alphanumeric()).collect()
    }

    #[macro_export]
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }
}
