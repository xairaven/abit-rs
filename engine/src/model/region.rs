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
