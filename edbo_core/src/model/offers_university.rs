use crate::dto::offers_university::OffersUniversityDto;
use crate::model::ModelError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug)]
pub struct OffersUniversity {
    pub university_id: u32,
    pub university_name: String,
    pub offers: Vec<u32>,
    pub offers_amount: i32,
}

impl TryFrom<OffersUniversityDto> for OffersUniversity {
    type Error = ModelError;

    fn try_from(dto: OffersUniversityDto) -> Result<Self, Self::Error> {
        let offers_dto = dto.ids.split(',').collect::<Vec<&str>>();

        if dto.n as usize != offers_dto.len() {
            return Err(Self::Error::OffersUniversityError(
                OffersUniversityError::WrongIdAmount,
            ));
        }

        let mut offers = vec![];
        for offer in offers_dto {
            let offer = offer.parse::<u32>().map_err(|err| {
                Self::Error::OffersUniversityError(
                    OffersUniversityError::FailedToParseId(err),
                )
            })?;
            offers.push(offer);
        }

        Ok(Self {
            university_id: dto.uid,
            university_name: dto.un,
            offers,
            offers_amount: dto.n as i32,
        })
    }
}

#[derive(Debug, Error)]
pub enum OffersUniversityError {
    #[error("Wrong ID amount")]
    WrongIdAmount,

    #[error("Failed to parse offer ID")]
    FailedToParseId(ParseIntError),
}
