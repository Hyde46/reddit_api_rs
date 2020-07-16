#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Clone)]
pub enum SortTime {
    hour,
    day,
    week,
    month,
    year,
    all,
}
impl std::fmt::Display for SortTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
