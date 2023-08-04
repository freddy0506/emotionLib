use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct SimpleSchueler {
    pub id: i64,
    pub first_name: String,
    pub last_name: String
}

#[derive(Debug, PartialEq, Serialize)]
pub struct SimpleVersuch {
    pub schueler_id : i32,
    pub wert: f32,
    pub kategorie_id: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NormVersuch {
    pub id: i64,
    pub schueler_id: i64,
    pub kategorie_id: i64,
    pub wert: f64,
    pub punkte: i64,
    pub ts_recording: i64,
    pub is_real: bool,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct PflichtKategorie {
    pub id: i64,
    pub done: bool,
    pub group_id: i64
}

#[derive(Debug, PartialEq, Serialize)]
pub struct SimpleKategorie {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Kategorie {
    pub id: i64,
    pub name: String,
    pub lauf: bool,
    pub einheit: char,
    pub max_vers: i64,
    pub messungs_form: String,
    pub kat_group_id: i64,
}