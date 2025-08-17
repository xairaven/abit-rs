use crate::model::institution::InstitutionError;

#[derive(Debug)]
pub enum Region {
    Vinnytsia,
    Volyn,
    Dnipropetrovsk,
    Donetsk,
    Zhytomyr,
    Zakarpattia,
    Zaporizhzhia,
    IvanoFrankivsk,
    Kyiv,
    Kirovohrad,
    Luhansk,
    Lviv,
    Mykolaiv,
    Odesa,
    Poltava,
    Rivne,
    Sumy,
    Ternopil,
    Kharkiv,
    Kherson,
    Khmelnytskyi,
    Cherkasy,
    Chernivtsi,
    Chernihiv,
    KyivCity,
}

impl TryFrom<&str> for Region {
    type Error = InstitutionError;

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
