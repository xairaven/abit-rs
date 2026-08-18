use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{Display, EnumString};

#[derive(Debug, IntoPrimitive, TryFromPrimitive, EnumString, Display)]
#[repr(u16)]
pub enum OwnershipForm {
    #[strum(serialize = "Державна")]
    State = 1,
    #[strum(serialize = "Комунальна")]
    Municipal = 2,
    #[strum(serialize = "Корпоративна")]
    Corporate = 3,
    #[strum(serialize = "Приватна")]
    Private = 4,
}
