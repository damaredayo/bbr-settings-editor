use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::{
    battlebit::{Axis, BattlebitType, Bool, Color, Float, Int, Key, State, Str},
    filters::{Filter, FilterVariant},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub typ: String,
    pub value: Value,
}

impl State {
    pub fn to_toml(&self) -> String {
        let mut entries = HashMap::new();
        for entry in self.registry.iter() {
            let name = entry.0;
            let typ = entry.1.to_reg_type_str();
            let value = entry.1.to_toml_value();

            let entry = Entry {
                typ: typ.to_string(),
                value: value.clone(),
            };

            entries.insert(name.clone(), entry);
        }

        let t = toml::Value::try_from(entries).expect("Failed to convert to TOML");
        let sorted = sort_toml(&t);
        toml::to_string_pretty(&sorted).expect("Failed to serialize to TOML")
    }

    pub fn to_filtered_toml(&self, include: Vec<FilterVariant>) -> String {
        let mut entries = HashMap::new();
        for entry in self.registry.iter() {
            let name = entry.0;
            let typ = entry.1.to_reg_type_str();

            if !include.iter().any(|filter| filter.filter(&name, typ)) {
                continue;
            }

            let value = entry.1.to_toml_value();

            let entry = Entry {
                typ: typ.to_string(),
                value: value.clone(),
            };

            entries.insert(name.clone(), entry);
        }

        let t = toml::Value::try_from(entries).expect("Failed to convert to TOML");
        let sorted = sort_toml(&t);
        toml::to_string_pretty(&sorted).expect("Failed to serialize to TOML")
    }

    pub fn from_toml_str(&mut self, toml: &str) -> Result<(), Box<dyn std::error::Error>> {
        let toml = toml::from_str(toml)?;
        self.from_toml(toml)
    }

    pub fn from_toml(&mut self, toml: Value) -> Result<(), Box<dyn std::error::Error>> {
        let entries: HashMap<String, Entry> = toml.try_into()?;

        let mut registry = HashMap::new();

        for (name, entry) in entries {
            let value = match entry.typ.as_str() {
                "int" => BattlebitType::Int(Int(entry
                    .value
                    .as_integer()
                    .ok_or("Invalid integer value")?
                    as i32)),
                "float" => BattlebitType::Float(Float(
                    entry.value.as_float().ok_or("Invalid float value")?,
                )),
                "bool" => {
                    BattlebitType::Bool(Bool(entry.value.as_bool().ok_or("Invalid bool value")?))
                }
                "axis" => BattlebitType::Axis(Axis(
                    entry.value.as_integer().ok_or("Invalid axis value")? as i32,
                )),
                "color" => {
                    let last = name.chars().last().ok_or("Invalid color name")?;
                    match last {
                        'r' => BattlebitType::Color(Color(
                            0,
                            entry.value.as_float().ok_or("Invalid color value")? as f64,
                        )),
                        'g' => BattlebitType::Color(Color(
                            1,
                            entry.value.as_float().ok_or("Invalid color value")? as f64,
                        )),
                        'b' => BattlebitType::Color(Color(
                            2,
                            entry.value.as_float().ok_or("Invalid color value")? as f64,
                        )),
                        'a' => BattlebitType::Color(Color(
                            3,
                            entry.value.as_float().ok_or("Invalid color value")? as f64,
                        )),
                        _ => return Err("Invalid color name".into()),
                    }
                }
                "key" => BattlebitType::Key(Key::from_key_ascii(
                    entry.value.as_str().ok_or("Invalid key value")?,
                )?),
                "str" => BattlebitType::Str(Str(entry
                    .value
                    .as_str()
                    .ok_or("Invalid string value")?
                    .to_string())),
                _ => return Err("Invalid type".into()),
            };

            registry.insert(name, value);
        }

        for (name, value) in registry {
            self.update_registry(&name, value);
        }

        Ok(())
    }
}

fn sort_toml(toml: &Value) -> Value {
    match toml {
        Value::Table(table) => {
            let sorted_map: BTreeMap<_, _> = table.iter().collect();
            let mut sorted_table = toml::value::Table::new();
            for (key, value) in sorted_map {
                sorted_table.insert(key.clone(), sort_toml(value));
            }
            Value::Table(sorted_table)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(sort_toml).collect()),
        _ => toml.clone(),
    }
}
