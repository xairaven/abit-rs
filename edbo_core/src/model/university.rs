#[derive(Debug)]
pub enum InstitutionCategory {
    // UA: Заклади вищої освіти
    HigherEducation = 1,

    // UA: Заклади професійної (професійно-технічної) освіти
    VocationalTechnical = 2,

    // UA: Заклади фахової передвищої освіти
    PreUniversityProfessional = 9,

    // UA: Наукові інститути (установи)
    Scientific = 8,

    // UA: Заклади післядипломної освіти
    Postgraduate = 10,
}
