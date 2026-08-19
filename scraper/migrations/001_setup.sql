-- COMMON
CREATE SCHEMA common;

CREATE TABLE IF NOT EXISTS common.institution_category (
    id INT2 PRIMARY KEY,
    title VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS common.ownership_form (
    id INT2 PRIMARY KEY,
    title VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS common.region (
    id INT2 PRIMARY KEY,
    title VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS common.institution (
    id INT2 PRIMARY KEY,
    name VARCHAR NOT NULL,
    parent_id INT2,
    short_name VARCHAR,
    english_name VARCHAR,
    is_from_crimea BOOLEAN NOT NULL,
    registration_date VARCHAR,
    category_id INT2 NOT NULL,
    ownership_form_id INT2 NOT NULL,
    region_id INT2,

    CONSTRAINT fk_institution_category FOREIGN KEY (category_id) REFERENCES common.institution_category(id),
    CONSTRAINT fk_institution_ownership FOREIGN KEY (ownership_form_id) REFERENCES common.ownership_form(id),
    CONSTRAINT fk_institution_region FOREIGN KEY (region_id) REFERENCES common.region(id)
);

CREATE TABLE IF NOT EXISTS common.application_status (
    id INT2 PRIMARY KEY,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS common.study_form (
    id INT2 PRIMARY KEY,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS common.knowledge_field (
    code CHAR PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS common.speciality (
    code VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    knowledge_field CHAR NOT NULL,

    CONSTRAINT fk_speciality_field FOREIGN KEY (knowledge_field) REFERENCES common.knowledge_field(code)
);

CREATE TABLE IF NOT EXISTS common.degree (
    id INT2 PRIMARY KEY,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS common.offer_type (
    id INT2 PRIMARY KEY,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS common.offer (
    id INTEGER PRIMARY KEY,
    title VARCHAR NOT NULL,
    degree_id INT2 NOT NULL,
    education_program VARCHAR NOT NULL,
    faculty VARCHAR,
    speciality_code VARCHAR NOT NULL,
    type_id INT2 NOT NULL,
    master_type VARCHAR,
    study_form_id INT2 NOT NULL,
    license_volume INTEGER NOT NULL,
    budgetary_places INTEGER NOT NULL,

    CONSTRAINT fk_offer_type FOREIGN KEY (type_id) REFERENCES common.offer_type(id),
    CONSTRAINT fk_offer_degree FOREIGN KEY (degree_id) REFERENCES common.degree(id),
    CONSTRAINT fk_offer_speciality FOREIGN KEY (speciality_code) REFERENCES common.speciality(code),
    CONSTRAINT fk_offer_study_form FOREIGN KEY (study_form_id) REFERENCES common.study_form(id)
);

CREATE TABLE IF NOT EXISTS common.offers_institutions (
    university_id INT2 NOT NULL,
    offer_id INTEGER NOT NULL,

    PRIMARY KEY (university_id, offer_id),
    CONSTRAINT fk_institution_many FOREIGN KEY (university_id) REFERENCES common.institution(id),
    CONSTRAINT fk_offer_many FOREIGN KEY (offer_id) REFERENCES common.offer(id)
);

-- SCRAPED
CREATE SCHEMA scraped;

CREATE TABLE IF NOT EXISTS scraped.applicant (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    grade_components JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS scraped.application (
    number_in_list INTEGER NOT NULL,
    status_id INT2 NOT NULL,
    grade DECIMAL (10, 3) NOT NULL,
    priority_code INT2 NOT NULL,

    offer_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,

    PRIMARY KEY (offer_id, number_in_list),
    CONSTRAINT fk_application_status FOREIGN KEY (status_id) REFERENCES common.application_status(id),
    CONSTRAINT fk_application_offer FOREIGN KEY (offer_id) REFERENCES common.offer(id),
    CONSTRAINT fk_application_user FOREIGN KEY (user_id) REFERENCES scraped.applicant(id)
);
