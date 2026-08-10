use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumIter;
use thiserror::Error;

#[derive(Debug, Copy, Clone, IntoPrimitive, TryFromPrimitive, EnumIter)]
#[repr(i8)]
pub enum Region {
    KyivCity = 1,
    Vinnytsia = 2,
    Volyn = 3,
    Dnipropetrovsk = 4,
    Donetsk = 5,
    Zhytomyr = 6,
    Zakarpattia = 7,
    Zaporizhzhia = 8,
    IvanoFrankivsk = 9,
    Kyiv = 10,
    Kirovohrad = 11,
    Luhansk = 12,
    Lviv = 13,
    Mykolaiv = 14,
    Odesa = 15,
    Poltava = 16,
    Rivne = 17,
    Sumy = 18,
    Ternopil = 19,
    Kharkiv = 20,
    Kherson = 21,
    Khmelnytskyi = 22,
    Cherkasy = 23,
    Chernivtsi = 24,
    Chernihiv = 25,
}

impl TryFrom<&str> for Region {
    type Error = RegionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Вінницька обл." => Ok(Self::Vinnytsia),
            "Волинська обл." => Ok(Self::Volyn),
            "Дніпропетровська обл." => Ok(Self::Dnipropetrovsk),
            "Донецька обл." => Ok(Self::Donetsk),
            "Житомирська обл." => Ok(Self::Zhytomyr),
            "Закарпатська обл." => Ok(Self::Zakarpattia),
            "Запорізька обл." => Ok(Self::Zaporizhzhia),
            "Івано-Франківська обл." => Ok(Self::IvanoFrankivsk),
            "Київська обл." => Ok(Self::Kyiv),
            "Кіровоградська обл." => Ok(Self::Kirovohrad),
            "Луганська обл." => Ok(Self::Luhansk),
            "Львівська обл." => Ok(Self::Lviv),
            "Миколаївська обл." => Ok(Self::Mykolaiv),
            "Одеська обл." => Ok(Self::Odesa),
            "Полтавська обл." => Ok(Self::Poltava),
            "Рівненська обл." => Ok(Self::Rivne),
            "Сумська обл." => Ok(Self::Sumy),
            "Тернопільська обл." => Ok(Self::Ternopil),
            "Харківська обл." => Ok(Self::Kharkiv),
            "Херсонська обл." => Ok(Self::Kherson),
            "Хмельницька обл." => Ok(Self::Khmelnytskyi),
            "Черкаська обл." => Ok(Self::Cherkasy),
            "Чернівецька обл." => Ok(Self::Chernivtsi),
            "Чернігівська обл." => Ok(Self::Chernihiv),
            "м. Київ" => Ok(Self::KyivCity),
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

        write!(f, "{text}")
    }
}

#[derive(Debug, Error)]
pub enum RegionError {
    #[error("Failed to parse region '{0}'")]
    UnknownRegion(String),
}
