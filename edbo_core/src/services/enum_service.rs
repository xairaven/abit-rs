use crate::database::Database;
use crate::error::CoreError;
use crate::repository::Repository;
use crate::repository::degree::DegreeRepository;
use crate::repository::institution_category::InstitutionCategoryRepository;
use crate::repository::knowledge_field::KnowledgeFieldRepository;
use crate::repository::offer_type::OfferTypeRepository;
use crate::repository::ownership_form::OwnershipFormRepository;
use crate::repository::priority::PriorityRepository;
use crate::repository::region::RegionRepository;
use crate::repository::speciality::SpecialityRepository;
use crate::repository::status::ApplicationStatusRepository;
use crate::repository::study_form::StudyFormRepository;
use crate::services::Service;

pub struct EnumService<'a> {
    application_status_repository: ApplicationStatusRepository<'a>,
    degrees_repository: DegreeRepository<'a>,
    institution_category_repository: InstitutionCategoryRepository<'a>,
    knowledge_field_repository: KnowledgeFieldRepository<'a>,
    offer_type_repository: OfferTypeRepository<'a>,
    ownership_form_repository: OwnershipFormRepository<'a>,
    priority_repository: PriorityRepository<'a>,
    region_repository: RegionRepository<'a>,
    speciality_repository: SpecialityRepository<'a>,
    study_form_repository: StudyFormRepository<'a>,
}

impl<'a> Service<'a> for EnumService<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        EnumService {
            application_status_repository: ApplicationStatusRepository::new(database),
            degrees_repository: DegreeRepository::new(database),
            institution_category_repository: InstitutionCategoryRepository::new(database),
            knowledge_field_repository: KnowledgeFieldRepository::new(database),
            offer_type_repository: OfferTypeRepository::new(database),
            ownership_form_repository: OwnershipFormRepository::new(database),
            priority_repository: PriorityRepository::new(database),
            region_repository: RegionRepository::new(database),
            speciality_repository: SpecialityRepository::new(database),
            study_form_repository: StudyFormRepository::new(database),
        }
    }
}

impl<'a> EnumService<'a> {
    pub async fn build(&self) -> Result<(), CoreError> {
        if self.application_status_repository.is_empty().await? {
            log::info!("Application statuses are empty, creating...");
            self.application_status_repository.create().await?;
        } else {
            log::info!("Application statuses are already populated, skipping...");
        }

        if self.degrees_repository.is_empty().await? {
            log::info!("Degrees are empty, creating...");
            self.degrees_repository.create().await?;
        } else {
            log::info!("Degrees  are already populated, skipping...");
        }

        if self.institution_category_repository.is_empty().await? {
            log::info!("Institution categories are empty, creating...");
            self.institution_category_repository.create().await?;
        } else {
            log::info!("Institution categories are already populated, skipping...");
        }

        if self.knowledge_field_repository.is_empty().await? {
            log::info!("Knowledge fields are empty, creating...");
            self.knowledge_field_repository.create().await?;
        } else {
            log::info!("Knowledge fields are already populated, skipping...");
        }

        if self.offer_type_repository.is_empty().await? {
            log::info!("Offer types are empty, creating...");
            self.offer_type_repository.create().await?;
        } else {
            log::info!("Offer types are already populated, skipping...");
        }

        if self.ownership_form_repository.is_empty().await? {
            log::info!("Ownership forms are empty, creating...");
            self.ownership_form_repository.create().await?;
        } else {
            log::info!("Ownership forms are already populated, skipping...");
        }

        if self.priority_repository.is_empty().await? {
            log::info!("Priority values are empty, creating...");
            self.priority_repository.create().await?;
        } else {
            log::info!("Priority values are already populated, skipping...");
        }

        if self.region_repository.is_empty().await? {
            log::info!("Region values are empty, creating...");
            self.region_repository.create().await?;
        } else {
            log::info!("Region values are already populated, skipping...");
        }

        if self.speciality_repository.is_empty().await? {
            log::info!("Specialities are empty, creating...");
            self.speciality_repository.create().await?;
        } else {
            log::info!("Specialities are already populated, skipping...");
        }

        if self.study_form_repository.is_empty().await? {
            log::info!("Study forms are empty, creating...");
            self.study_form_repository.create().await?;
        } else {
            log::info!("Study forms are already populated, skipping...");
        }

        Ok(())
    }
}
