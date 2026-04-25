use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderId {
    Brave,
    Exa,
}

impl fmt::Display for ProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderId::Brave => formatter.write_str("brave"),
            ProviderId::Exa => formatter.write_str("exa"),
        }
    }
}
