#[derive(Debug)]
pub enum StudyForm {
    FullTime = 1,
    External = 2,
    Evening = 4,
    Online = 5,
}

impl std::fmt::Display for StudyForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::FullTime => "Денна",
            Self::External => "Заочна",
            Self::Evening => "Вечірня",
            Self::Online => "Дистанційна",
        };

        write!(f, "{}", text)
    }
}
