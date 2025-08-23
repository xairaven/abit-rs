use crate::model::application::{Application, GradeComponent};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Applicant {
    pub id: i32,
    pub name: String,
    pub grade_components: Vec<GradeComponent>,
}

pub fn list(applications: &mut [Application]) -> Vec<Applicant> {
    let mut map: HashMap<String, Vec<Applicant>> = HashMap::new();
    let mut id_counter = 1;

    for application in applications.iter_mut() {
        let applicants = map.entry(application.full_name.clone()).or_default();

        let mut found = false;
        for applicant in applicants.iter() {
            if is_same_person_by_grades(application, applicant) {
                application.user_id = Some(applicant.id);
                found = true;
                break;
            }
        }

        if !found {
            let new_applicant = Applicant {
                id: id_counter,
                name: application.full_name.clone(),
                grade_components: application.grade_components.clone(),
            };
            application.user_id = Some(id_counter);
            applicants.push(new_applicant);
            id_counter += 1;
        }
    }

    let mut applicants: Vec<Applicant> = Vec::new();
    for values in map.values_mut() {
        applicants.append(values);
    }

    applicants
}

fn is_same_person_by_grades(application: &Application, applicant: &Applicant) -> bool {
    const MUST_EQUAL: usize = 2;
    let mut equal_count = 0;
    let mut exclude_indexes: Vec<usize> = Vec::new();
    for grade_person in &applicant.grade_components {
        for (i, grade_application) in application.grade_components.iter().enumerate() {
            if exclude_indexes.contains(&i) {
                continue;
            }
            if grade_person.0 == grade_application.0 {
                equal_count += 1;
                exclude_indexes.push(i);
                break;
            }
        }
    }

    MUST_EQUAL >= equal_count
}
