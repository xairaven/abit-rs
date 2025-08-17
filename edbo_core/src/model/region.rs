use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Display)]
pub enum Region {
    #[strum(serialize = "Вінницька область")]
    Vinnytsia,

    #[strum(serialize = "Волинська область")]
    Volyn,

    #[strum(serialize = "Дніпропетровська область")]
    Dnipropetrovsk,

    #[strum(serialize = "Донецька область")]
    Donetsk,

    #[strum(serialize = "Житомирська область")]
    Zhytomyr,

    #[strum(serialize = "Закарпатська область")]
    Zakarpattia,

    #[strum(serialize = "Запорізька область")]
    Zaporizhzhia,

    #[strum(serialize = "Івано-Франківська область")]
    IvanoFrankivsk,

    #[strum(serialize = "Київська область")]
    Kyiv,

    #[strum(serialize = "Кіровоградська область")]
    Kirovohrad,

    #[strum(serialize = "Луганська область")]
    Luhansk,

    #[strum(serialize = "Львівська область")]
    Lviv,

    #[strum(serialize = "Миколаївська область")]
    Mykolaiv,

    #[strum(serialize = "Одеська область")]
    Odesa,

    #[strum(serialize = "Полтавська область")]
    Poltava,

    #[strum(serialize = "Рівненська область")]
    Rivne,

    #[strum(serialize = "Сумська область")]
    Sumy,

    #[strum(serialize = "Тернопільська область")]
    Ternopil,

    #[strum(serialize = "Харківська область")]
    Kharkiv,

    #[strum(serialize = "Херсонська область")]
    Kherson,

    #[strum(serialize = "Хмельницька область")]
    Khmelnytskyi,

    #[strum(serialize = "Черкаська область")]
    Cherkasy,

    #[strum(serialize = "Чернівецька область")]
    Chernivtsi,

    #[strum(serialize = "Чернігівська область")]
    Chernihiv,

    #[strum(serialize = "м. Київ")]
    KyivCity,
}
