//Déclarer les types et interfaces pour les données utilisateur
//Typescript panique si la valeur est null ou undefined

export type UserRole = "pending" | "temp" | "owner" | "admin";

export interface FullProfileData {
    user: UserData; // générique pour tous
    temp_data?: TempFullProfile; // défini si user.role === "temp"
    owner_data?: OwnerFullProfile; // défini si user.role === "owner"
}

export interface UserData {
    id?: string;
    email: string;
    role: UserRole;
}

// ---------- TEMP PROFILE STRUCTURES ----------

export interface TempFullProfile {
    temp_info: TempData;
    availabilities: Availability[];
    conditions: WorkCondition[];
    diplomas: Diploma[];
    experiences: Experience[];
    skills: Skill[];
    documents: Document[];
}

export interface TempData {
    first_name: string,
    last_name: string,
    address: string;
    phone: string;
    birth_date: string;
    driver_license: boolean;
    transport_modes: string;
}

export interface Availability {
    available_periods: string;
    work_hours: string;
    preferred_locations: string;
    max_travel_time: string;
}

export interface WorkCondition {
    hourly_rate: string;
    contract_types: string; // e.g. "CDD", "CDI"
    self_employment: string;
}

export interface Diploma {
    diploma_name: string;
    other_certifications: string;
    year_obtained: string;
    institution: string;
}

export interface Experience {
    total_experience: string;
    previous_jobs: string;
    structure_types: string;
    tasks: string;
}

export interface Skill {
    languages: string,
    pedagogies: string,
    special_skills: string,
    special_needs_handling: string,
}

export interface Document {
    motivation_letter: string,
    professional_refs: string,
    required_docs: string,
    criminal_record: string,
    diplomas: string,
}

//type spécifique pour le formulaire d'inscription
export interface IntervenantFormData {
    temp_info: TempData;
    availabilities: Availability[];
    conditions: WorkCondition[];
    diplomas: Diploma[];
    documents: Document[];
    experiences: Experience[];
    skills: Skill[];
}

// ---------- OWNER PROFILE STRUCTURES ----------

export interface OwnerFullProfile {
    owner: OwnerData;
    creches: CrecheDocument[];
}

export interface OwnerData {
    full_name: string;
    phone: string;
    address: string;
    birth_date: string;
    creche_name: string;
    creche_address: string;
    siret: string;
    has_employees: boolean;
    needs: string[]; // par ex. ["remplacement ponctuel", "poste à pourvoir", etc.]
    documents: CrecheDocument[];
}

export interface CrecheDocument {
    document_type: string;
    file_url: string;
    uploaded_at: string;
}