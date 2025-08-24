SELECT 'CREATE DATABASE edbo_db'
    WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'edbo_db');

CREATE TABLE IF NOT EXISTS institution_category (
    id INT2 PRIMARY KEY,
    description VARCHAR NOT NULL,
    code INT2
);

CREATE TABLE IF NOT EXISTS ownership_form (
    id INT2 PRIMARY KEY,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS region (
    id INT2 PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS institution (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    parent_id INTEGER,
    short_name VARCHAR,
    english_name VARCHAR,
    is_from_crimea BOOLEAN NOT NULL,
    registration_year INT2,
    category_id INT2 NOT NULL,
    ownership_form_id INT2 NOT NULL,
    region_id INT2 NOT NULL,

    CONSTRAINT fk_institution_category FOREIGN KEY (category_id) REFERENCES institution_category(id),
    CONSTRAINT fk_institution_ownership FOREIGN KEY (ownership_form_id) REFERENCES ownership_form(id),
    CONSTRAINT fk_institution_region FOREIGN KEY (region_id) REFERENCES region(id)
);

CREATE TABLE IF NOT EXISTS application_status (
    id INT2 PRIMARY KEY,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS study_form (
    id INT2 PRIMARY KEY,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS knowledge_field (
    code CHAR PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS speciality (
    code VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    knowledge_field CHAR NOT NULL,

    CONSTRAINT fk_speciality_field FOREIGN KEY (knowledge_field) REFERENCES knowledge_field(code)
);

CREATE TABLE IF NOT EXISTS degree (
    id INTEGER PRIMARY KEY,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS offer (
    id INTEGER PRIMARY KEY,
    title VARCHAR NOT NULL,
    degree_id INTEGER NOT NULL,
    education_program VARCHAR NOT NULL,
    study_form_id INT2 NOT NULL,
    faculty VARCHAR,
    speciality_code VARCHAR NOT NULL,
    master_type VARCHAR,

    license_volume INTEGER NOT NULL,
    budgetary_places INTEGER NOT NULL,

    CONSTRAINT fk_offer_degree FOREIGN KEY (degree_id) REFERENCES degree(id),
    CONSTRAINT fk_offer_speciality FOREIGN KEY (speciality_code) REFERENCES speciality(code),
    CONSTRAINT fk_offer_study_form FOREIGN KEY (study_form_id) REFERENCES study_form(id)
);

CREATE TABLE IF NOT EXISTS offers_institutions (
    university_id INTEGER NOT NULL,
    offer_id INTEGER NOT NULL,

    PRIMARY KEY (university_id, offer_id),
    CONSTRAINT fk_institution_many FOREIGN KEY (university_id) REFERENCES institution(id),
    CONSTRAINT fk_offers_many FOREIGN KEY (offer_id) REFERENCES offer(id)
);

CREATE TABLE IF NOT EXISTS applicant (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    grade_components JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS priority (
    id INT2 PRIMARY KEY,
    key VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS application (
    number_in_list INTEGER NOT NULL,
    status_id INT2 NOT NULL,
    grade DECIMAL (10, 3) NOT NULL,
    priority_id INT2 NOT NULL,

    offer_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,

    PRIMARY KEY (offer_id, number_in_list),
    CONSTRAINT fk_application_status FOREIGN KEY (status_id) REFERENCES application_status(id),
    CONSTRAINT fk_application_priority FOREIGN KEY (priority_id) REFERENCES priority(id),
    CONSTRAINT fk_application_offer FOREIGN KEY (offer_id) REFERENCES offer(id),
    CONSTRAINT fk_application_user FOREIGN KEY (user_id) REFERENCES applicant(id)
);
