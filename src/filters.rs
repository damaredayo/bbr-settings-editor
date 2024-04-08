use lazy_static::lazy_static;

pub trait Filter {
    fn filter(&self, input: &str, typ: &str) -> bool;
}

#[derive(Clone)]
pub struct Hitmarkers;
impl Filter for Hitmarkers {
    fn filter(&self, input: &str, _: &str) -> bool {
        input.contains("HitMarker")
    }
}

#[derive(Clone)]
pub struct Keybindings;
impl Filter for Keybindings {
    fn filter(&self, input: &str, typ: &str) -> bool {
        input.contains("key") || typ == "key" || input.contains("axis") || typ == "axis"
    }
}

#[derive(Clone)]
pub struct Sensitivity;
impl Filter for Sensitivity {
    fn filter(&self, input: &str, _: &str) -> bool {
        input.contains("Sensitivity")
    }
}

#[derive(Clone)]
pub struct Audio;
impl Filter for Audio {
    fn filter(&self, input: &str, _: &str) -> bool {
        input.contains("Volume")
    }
}

#[derive(Clone)]
pub struct Manual(String);
impl Filter for Manual {
    fn filter(&self, input: &str, _: &str) -> bool {
        input.contains(&self.0)
    }
}

#[derive(Clone)]
pub enum FilterVariant {
    Hitmarkers(Hitmarkers),
    Keybindings(Keybindings),
    Sensitivity(Sensitivity),
    Audio(Audio),
    Manual(Manual),
}

impl Filter for FilterVariant {
    fn filter(&self, input: &str, typ: &str) -> bool {
        match self {
            FilterVariant::Hitmarkers(f) => f.filter(input, typ),
            FilterVariant::Keybindings(f) => f.filter(input, typ),
            FilterVariant::Sensitivity(f) => f.filter(input, typ),
            FilterVariant::Audio(f) => f.filter(input, typ),
            FilterVariant::Manual(f) => f.filter(input, typ),
        }
    }
}

lazy_static! {
    pub static ref COMMON_FILTERS: Vec<FilterVariant> = vec![
        FilterVariant::Hitmarkers(Hitmarkers),
        FilterVariant::Keybindings(Keybindings),
        FilterVariant::Audio(Audio),
    ];
}

pub fn parse_filters(filters: Vec<String>) -> Vec<FilterVariant> {
    let mut result = Vec::new();

    for f in filters {
        let filter = match f.as_str() {
            "hitmarkers" => FilterVariant::Hitmarkers(*Box::new(Hitmarkers)),
            "keybindings" => FilterVariant::Keybindings(*Box::new(Keybindings)),
            "sensitivity" => FilterVariant::Sensitivity(*Box::new(Sensitivity)),
            "audio" => FilterVariant::Audio(*Box::new(Audio)),
            "common" => {
                result.append(COMMON_FILTERS.to_vec().as_mut());
                continue;
            }
            _ => FilterVariant::Manual(*Box::new(Manual(f.to_string()))),
        };

        result.push(filter);
    }

    result
}
