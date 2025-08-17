use thiserror::Error;

// Source: https://zakon.rada.gov.ua/laws/show/266-2015-п#n11

#[derive(PartialEq)]
pub enum KnowledgeField {
    // UA: (A) Освіта
    Education,

    // UA: (B) Культура, мистецтво та гуманітарні науки
    CultureArtsHumanities,

    // UA: (C) Соціальні науки, журналістика, інформація та міжнародні відносини
    SocialSciences,

    // UA: (D) Бізнес, адміністрування та право
    BusinessAdministrationLaw,

    // UA: (E) Природничі науки, математика та статистика
    NaturalSciencesMathematics,

    // UA: (F) Інформаційні технології
    InformationTechnologies,

    // UA: (G) Інженерія, виробництво та будівництво
    EngineeringManufacturingConstruction,

    // UA: (H) Сільське, лісове, рибне господарство та ветеринарна медицина
    AgricultureForestryFisheriesVeterinary,

    // UA: (I) Охорона здоров’я та соціальне забезпечення
    HealthcareSocialSecurity,

    // UA: (J) Транспорт та послуги
    TransportServices,

    // UA: (K) Безпека та оборона
    SecurityDefense,
}

impl TryFrom<&str> for KnowledgeField {
    type Error = SpecialityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::Education),
            "B" => Ok(Self::CultureArtsHumanities),
            "C" => Ok(Self::SocialSciences),
            "D" => Ok(Self::BusinessAdministrationLaw),
            "E" => Ok(Self::NaturalSciencesMathematics),
            "F" => Ok(Self::InformationTechnologies),
            "G" => Ok(Self::EngineeringManufacturingConstruction),
            "H" => Ok(Self::AgricultureForestryFisheriesVeterinary),
            "I" => Ok(Self::HealthcareSocialSecurity),
            "J" => Ok(Self::TransportServices),
            "K" => Ok(Self::SecurityDefense),

            _ => Err(Self::Error::UnknownKnowledgeField(value.to_string())),
        }
    }
}

#[derive(Debug, Error)]
pub enum SpecialityError {
    #[error("Unknown knowledge field: {0}")]
    UnknownKnowledgeField(String),

    #[error("Unknown speciality code: {0}")]
    UnknownSpecialityCode(String),
}

macro_rules! define_specialities {
    (
        $(
            $field:ident {
                $( { code: $code:literal, ua: $ua:literal, $variant:ident } )*
                $(,)?
            } $(,)?
        )*
    ) => {
      #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum Speciality {
            $(
                $(
                    $variant,
                )*
            )*
        }

        impl Speciality {
            pub fn code(&self) -> &'static str {
                match self {
                    $(
                        $(
                            Speciality::$variant => $code,
                        )*
                    )*
                }
            }

            pub fn title(&self) -> &'static str {
                match self {
                    $(
                        $(
                            Speciality::$variant => $ua,
                        )*
                    )*
                }
            }
        }

        impl From<&Speciality> for KnowledgeField {
            fn from(value: &Speciality) -> KnowledgeField {
                match value {
                    $(
                        $(
                            Speciality::$variant => KnowledgeField::$field,
                        )*
                    )*
                }
            }
        }

        impl TryFrom<&str> for Speciality {
            type Error = SpecialityError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $(
                            $code => Ok(Speciality::$variant),
                        )*
                    )*
                    _ => Err(SpecialityError::UnknownSpecialityCode(value.to_string())),
                }
            }
        }

        pub const ALL_SPECIALITIES: &[( KnowledgeField, Speciality)] = &[
            $(
                $(
                    (KnowledgeField::$field, Speciality::$variant),
                )*
            )*
        ];

        impl KnowledgeField {
            pub fn specialities(&self) -> Vec<Speciality> {
                ALL_SPECIALITIES.iter()
                    .filter(|(field, _)| field == self)
                    .map(|(_, s)| *s)
                    .collect()
            }
        }
    };
}

define_specialities! {
    // UA: (A) Освіта
    Education {
       { code: "A1", ua: "Освітні науки", EducationScience }
       { code: "A2", ua: "Дошкільна освіта", TrainingPreSchoolTeachers }
       { code: "A3", ua: "Початкова освіта", BasicEducation }
       { code: "A4", ua: "Середня освіта (за предметними спеціальностями)", SecondaryEducationBySpecialization }
       { code: "A5", ua: "Професійна освіта (за спеціалізаціями)", ProfessionalEducationBySpecialization }
       { code: "A6", ua: "Спеціальна освіта (за спеціалізаціями)", SpecialEducationBySpecialization }
       { code: "A7", ua: "Фізична культура і спорт", PhysicalEducationSports }
    },

    // UA: (B) Культура, мистецтво та гуманітарні науки
    CultureArtsHumanities {
        { code: "B1", ua: "Аудіовізуальне мистецтво та медіавиробництво", MediaProduction }
        { code: "B2", ua: "Дизайн", Design }
        { code: "B3", ua: "Декоративне мистецтво та ремесла", Handicrafts }
        { code: "B4", ua: "Образотворче мистецтво та реставрація", FineArts }
        { code: "B5", ua: "Музичне мистецтво", MusicArts }
        { code: "B6", ua: "Перформативні мистецтва", PerformingArts }
        { code: "B7", ua: "Релігієзнавство", ReligiousStudies }
        { code: "B8", ua: "Богослов’я", Theology }
        { code: "B9", ua: "Історія та археологія", HistoryArchaeology }
        { code: "B10", ua: "Філософія", PhilosophyEthics }
        { code: "B11", ua: "Філологія (за спеціалізаціями)", LanguageAcquisition }
        { code: "B12", ua: "Культурологія та музеєзнавство", SociologyCulturalStudies }
        { code: "B13", ua: "Бібліотечна, інформаційна та архівна справа", LibraryArchivalStudies }
        { code: "B14", ua: "Організація соціокультурної діяльності", OrganizationSocioCulturalActivities }
    },

    // UA: (C) Соціальні науки, журналістика, інформація та міжнародні відносини
    SocialSciences {
        { code: "C1", ua: "Економіка та міжнародні економічні відносини (за спеціалізаціями)", Economics }
        { code: "C2", ua: "Політологія", PoliticalSciences }
        { code: "C3", ua: "Міжнародні відносини", InternationalRelations }
        { code: "C4", ua: "Психологія", Psychology }
        { code: "C5", ua: "Соціологія", Sociology }
        { code: "C6", ua: "Географія та регіональні студії", Geography }
        { code: "C7", ua: "Журналістика", Journalism }
    },

    // UA: (D) Бізнес, адміністрування та право
    BusinessAdministrationLaw {
        { code: "D1", ua: "Облік і оподаткування", AccountingTaxation }
        { code: "D2", ua: "Фінанси, банківська справа, страхування та фондовий ринок", FinanceBankingInsurance }
        { code: "D3", ua: "Менеджмент", Management }
        { code: "D4", ua: "Публічне управління та адміністрування", Administration }
        { code: "D5", ua: "Маркетинг", Marketing }
        { code: "D6", ua: "Секретарська та офісна справа", SecretarialWork }
        { code: "D7", ua: "Торгівля", Sales }
        { code: "D8", ua: "Право", Law }
        { code: "D9", ua: "Міжнародне право", InternationalLaw }
    }

    // UA: (E) Природничі науки, математика та статистика
    NaturalSciencesMathematics {
        { code: "E1", ua: "Біологія та біохімія", Biology }
        { code: "E2", ua: "Екологія", EnvironmentalSciences }
        { code: "E3", ua: "Хімія", Chemistry }
        { code: "E4", ua: "Науки про Землю", EarthSciences }
        { code: "E5", ua: "Фізика та астрономія", Physics }
        { code: "E6", ua: "Прикладна фізика та наноматеріали", AppliedPhysics }
        { code: "E7", ua: "Математика", Mathematics }
        { code: "E8", ua: "Статистика", Statistics }
    }

    // UA: (F) Інформаційні технології
    InformationTechnologies {
        { code: "F1", ua: "Прикладна математика", AppliedMathematics }
        { code: "F2", ua: "Інженерія програмного забезпечення", SoftwareEngineering }
        { code: "F3", ua: "Комп’ютерні науки", ComputerSciences }
        { code: "F4", ua: "Системний аналіз та наука про дані", SystemAnalysis }
        { code: "F5", ua: "Кібербезпека та захист інформації", Cybersecurity }
        { code: "F6", ua: "Інформаційні системи і технології", InformationSystemsTechnologies }
        { code: "F7", ua: "Комп’ютерна інженерія", ComputerEngineering }
    }

    // UA: (G) Інженерія, виробництво та будівництво
    // UA: (H) Сільське, лісове, рибне господарство та ветеринарна медицина
    // UA: (I) Охорона здоров’я та соціальне забезпечення
    // UA: (J) Транспорт та послуги
    // UA: (K) Безпека та оборона
}
