pub use xml_library::XMLLibrary;
pub use mut_field::MutField;

pub struct Point {
    lat: f32,
    lon: f32,
}

mod xml_document;
mod xml_library;
mod mut_hash_map;
mod mut_field;
mod default_id;
