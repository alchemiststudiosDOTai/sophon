#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    Web,
    News,
    Images,
    Videos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafeSearch {
    Off,
    Moderate,
    Strict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TimeRange {
    Day,
    Week,
    Month,
    Year,
}
