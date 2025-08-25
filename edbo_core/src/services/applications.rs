use crate::api;
use crate::database::Database;
use crate::error::CoreError;
use crate::model::applicant::Applicant;
use crate::model::application::Application;
use crate::model::offer::Offer;
use crate::repository::Repository;
use crate::repository::applicant::ApplicantRepository;
use crate::repository::application::ApplicationRepository;
use crate::services::Service;

pub struct ApplicationService<'a> {
    application_repo: ApplicationRepository<'a>,
    applicant_repo: ApplicantRepository<'a>,
}

impl<'a> Service<'a> for ApplicationService<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self {
            application_repo: ApplicationRepository::new(database),
            applicant_repo: ApplicantRepository::new(database),
        }
    }
}

impl<'a> ApplicationService<'a> {
    pub async fn get(
        &self, offers: &[Offer],
    ) -> Result<(Vec<Application>, Vec<Applicant>), CoreError> {
        if self.application_repo.is_empty().await?
            || self.applicant_repo.is_empty().await?
        {
            self.application_repo.truncate().await?;
            self.applicant_repo.truncate().await?;
            log::info!(
                "Both applicant and application tables are clear. Requesting data from API..."
            );
            let (applications, applicants) = api::applications::list(offers).await?;
            let applicants = applicants.to_vec();
            for application in applications.iter() {
                self.application_repo.create(application).await?;
            }
            log::info!(
                "Application table populated with {} records",
                applications.len()
            );
            for applicant in applicants.iter() {
                self.applicant_repo.create(applicant).await?;
            }
            log::info!(
                "Applicant table populated with {} records",
                applicants.len()
            );
            Ok((applications, applicants))
        } else {
            log::info!(
                "Both applicant and application tables are already populated. Trying to fetch from DB..."
            );
            let applications = self.application_repo.find_all().await?;
            log::info!(
                "Successfully fetched {} applications records from DB",
                applications.len()
            );
            let applicants = self.applicant_repo.find_all().await?;
            log::info!(
                "Successfully fetched {} applicants records from DB",
                applicants.len()
            );
            Ok((applications, applicants))
        }
    }
}
