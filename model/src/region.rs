use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(i8)]
pub enum Region {
    KyivCity = 80,
    Vinnytsia = 5,
    Volyn = 7,
    Dnipropetrovsk = 12,
    Donetsk = 14,
    Zhytomyr = 18,
    Zakarpattia = 21,
    Zaporizhzhia = 23,
    IvanoFrankivsk = 26,
    Kyiv = 32,
    Kirovohrad = 35,
    Luhansk = 44,
    Lviv = 46,
    Mykolaiv = 48,
    Odesa = 51,
    Poltava = 53,
    Rivne = 56,
    Sumy = 59,
    Ternopil = 61,
    Kharkiv = 63,
    Kherson = 65,
    Khmelnytskyi = 68,
    Cherkasy = 71,
    Chernivtsi = 73,
    Chernihiv = 74,
}
