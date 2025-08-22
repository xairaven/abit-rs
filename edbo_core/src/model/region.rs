use thiserror::Error;

#[derive(Debug)]
pub enum Region {
    Vinnytsia = 1,
    Volyn = 2,
    Dnipropetrovsk = 3,
    Donetsk = 4,
    Zhytomyr = 5,
    Zakarpattia = 6,
    Zaporizhzhia = 7,
    IvanoFrankivsk = 8,
    Kyiv = 9,
    Kirovohrad = 10,
    Luhansk = 11,
    Lviv = 12,
    Mykolaiv = 13,
    Odesa = 14,
    Poltava = 15,
    Rivne = 16,
    Sumy = 17,
    Ternopil = 18,
    Kharkiv = 19,
    Kherson = 20,
    Khmelnytskyi = 21,
    Cherkasy = 22,
    Chernivtsi = 23,
    Chernihiv = 24,
    KyivCity = 25,
}

impl TryFrom<&str> for Region {
    type Error = RegionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Вінницька обл." => Ok(Region::Vinnytsia),
            "Волинська обл." => Ok(Region::Volyn),
            "Дніпропетровська обл." => Ok(Region::Dnipropetrovsk),
            "Донецька обл." => Ok(Region::Donetsk),
            "Житомирська обл." => Ok(Region::Zhytomyr),
            "Закарпатська обл." => Ok(Region::Zakarpattia),
            "Запорізька обл." => Ok(Region::Zaporizhzhia),
            "Івано-Франківська обл." => Ok(Region::IvanoFrankivsk),
            "Київська обл." => Ok(Region::Kyiv),
            "Кіровоградська обл." => Ok(Region::Kirovohrad),
            "Луганська обл." => Ok(Region::Luhansk),
            "Львівська обл." => Ok(Region::Lviv),
            "Миколаївська обл." => Ok(Region::Mykolaiv),
            "Одеська обл." => Ok(Region::Odesa),
            "Полтавська обл." => Ok(Region::Poltava),
            "Рівненська обл." => Ok(Region::Rivne),
            "Сумська обл." => Ok(Region::Sumy),
            "Тернопільська обл." => Ok(Region::Ternopil),
            "Харківська обл." => Ok(Region::Kharkiv),
            "Херсонська обл." => Ok(Region::Kherson),
            "Хмельницька обл." => Ok(Region::Khmelnytskyi),
            "Черкаська обл." => Ok(Region::Cherkasy),
            "Чернівецька обл." => Ok(Region::Chernivtsi),
            "Чернігівська обл." => Ok(Region::Chernihiv),
            "м. Київ" => Ok(Region::KyivCity),
            _ => Err(Self::Error::UnknownRegion(value.to_string())),
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Vinnytsia => "Вінницька обл.",
            Self::Volyn => "Волинська обл.",
            Self::Dnipropetrovsk => "Дніпропетровська обл.",
            Self::Donetsk => "Донецька обл.",
            Self::Zhytomyr => "Житомирська обл.",
            Self::Zakarpattia => "Закарпатська обл.",
            Self::Zaporizhzhia => "Запорізька обл.",
            Self::IvanoFrankivsk => "Івано-Франківська обл.",
            Self::Kyiv => "Київська обл.",
            Self::Kirovohrad => "Кіровоградська обл.",
            Self::Luhansk => "Луганська обл.",
            Self::Lviv => "Львівська обл.",
            Self::Mykolaiv => "Миколаївська обл.",
            Self::Odesa => "Одеська обл.",
            Self::Poltava => "Полтавська обл.",
            Self::Rivne => "Рівненська обл.",
            Self::Sumy => "Сумська обл.",
            Self::Ternopil => "Тернопільська обл.",
            Self::Kharkiv => "Харківська обл.",
            Self::Kherson => "Херсонська обл.",
            Self::Khmelnytskyi => "Хмельницька обл.",
            Self::Cherkasy => "Черкаська обл.",
            Self::Chernivtsi => "Чернівецька обл.",
            Self::Chernihiv => "Чернігівська обл.",
            Self::KyivCity => "м. Київ",
        };

        write!(f, "{}", text)
    }
}

#[derive(Debug, Error)]
pub enum RegionError {
    #[error("Failed to parse region '{0}'")]
    UnknownRegion(String),
}
