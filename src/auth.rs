#[derive(Debug, Clone, Default)]
pub enum Authorization {
    Bearer(String),
    #[default]
    None,
}
