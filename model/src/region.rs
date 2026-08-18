use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{Display, EnumString};

#[derive(Debug, IntoPrimitive, TryFromPrimitive, EnumString, Display)]
#[repr(i16)]
pub enum Region {
    Every = 0,

    #[strum(serialize = "Київ")]
    KyivCity = 80,
    #[strum(serialize = "Вінницька")]
    Vinnytsia = 5,
    #[strum(serialize = "Волинська")]
    Volyn = 7,
    #[strum(serialize = "Дніпропетровська")]
    Dnipropetrovsk = 12,
    #[strum(serialize = "Донецька")]
    Donetsk = 14,
    #[strum(serialize = "Житомирська")]
    Zhytomyr = 18,
    #[strum(serialize = "Закарпатська")]
    Zakarpattia = 21,
    #[strum(serialize = "Запорізька")]
    Zaporizhzhia = 23,
    #[strum(serialize = "Івано-Франківська")]
    IvanoFrankivsk = 26,
    #[strum(serialize = "Київська")]
    Kyiv = 32,
    #[strum(serialize = "Кіровоградська")]
    Kirovohrad = 35,
    #[strum(serialize = "Луганська")]
    Luhansk = 44,
    #[strum(serialize = "Львівська")]
    Lviv = 46,
    #[strum(serialize = "Миколаївська")]
    Mykolaiv = 48,
    #[strum(serialize = "Одеська")]
    Odesa = 51,
    #[strum(serialize = "Полтавська")]
    Poltava = 53,
    #[strum(serialize = "Рівненська")]
    Rivne = 56,
    #[strum(serialize = "Сумська")]
    Sumy = 59,
    #[strum(serialize = "Тернопільська")]
    Ternopil = 61,
    #[strum(serialize = "Харківська")]
    Kharkiv = 63,
    #[strum(serialize = "Херсонська")]
    Kherson = 65,
    #[strum(serialize = "Хмельницька")]
    Khmelnytskyi = 68,
    #[strum(serialize = "Черкаська")]
    Cherkasy = 71,
    #[strum(serialize = "Чернівецька")]
    Chernivtsi = 73,
    #[strum(serialize = "Чернігівська")]
    Chernihiv = 74,
}
