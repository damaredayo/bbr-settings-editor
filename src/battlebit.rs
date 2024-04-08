use std::collections::HashMap;

use toml::Value;
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_SET_VALUE},
    RegKey, RegValue,
};

const BBR_REG_SUBKEY: &str = "SOFTWARE\\BattleBitDevTeam\\BattleBit";

#[derive(Debug)]
pub struct Int(pub i32);

#[derive(Debug)]
pub struct Float(pub f64);

#[derive(Debug)]
pub struct Bool(pub bool);

#[derive(Debug)]
pub struct Axis(pub i32);

#[derive(Debug)]
pub struct Color(pub i32, pub f64); // param 1: R: 0, G: 1, B: 2, A: 3, param 2: value

#[derive(Debug)]
pub struct Key(pub i32);

impl Key {
    pub fn to_key_ascii(&self) -> String {
        unsafe { std::char::from_u32_unchecked(self.0 as u32) }.to_string()
    }

    pub fn from_key_ascii(key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let key = if key.starts_with("\\u") {
            let key = key.trim_start_matches("\\u");
            let key = u32::from_str_radix(key, 16)?;
            char::from_u32(key)
                .ok_or("Failed to convert u32 to char")?
                .to_string()
        } else {
            key.to_string()
        };

        let key = key.chars().next().ok_or("No char found")? as i32;
        Ok(Key(key))
    }
}

#[derive(Debug)]
pub struct Str(pub String);

#[derive(Debug)]
pub enum BattlebitType {
    Int(Int),
    Float(Float),
    Bool(Bool),
    Axis(Axis),
    Color(Color),
    Key(Key),
    Str(Str),
}

impl BattlebitType {
    pub fn from_reg(
        key: String,
        value: RegValue,
    ) -> Result<(String, BattlebitType), Box<dyn std::error::Error>> {
        let key_parts: Vec<&str> = key.split('_').collect();
        let length = key_parts.len();

        let typ = key_parts[length - 2];
        let name = key_parts[..length - 2].join("_");

        if typ.starts_with("Screenmanager") {
            let value = i32::from_le_bytes(match value.bytes.to_vec().try_into() {
                Ok(v) => v,
                Err(_) => return Ok((name, BattlebitType::Int(Int(0)))),
            });
            return Ok((name, BattlebitType::Int(Int(value as i32))));
        }

        match typ {
            "int" => {
                let value = i32::from_le_bytes(match value.bytes.to_vec().try_into() {
                    Ok(v) => v,
                    Err(_) => return Ok((name, BattlebitType::Int(Int(0)))),
                });
                Ok((name, BattlebitType::Int(Int(value as i32))))
            }
            "float" => {
                let value = f64::from_le_bytes(match value.bytes.to_vec().try_into() {
                    Ok(v) => v,
                    Err(_) => return Ok((name, BattlebitType::Float(Float(0.0)))),
                });
                Ok((name, BattlebitType::Float(Float(value as f64))))
            }
            "bool" => {
                let value = i32::from_le_bytes(match value.bytes.to_vec().try_into() {
                    Ok(v) => v,
                    Err(_) => return Ok((name, BattlebitType::Bool(Bool(false)))),
                });
                Ok((name, BattlebitType::Bool(Bool(value == 1))))
            }
            "axis" => {
                let value = i32::from_le_bytes(match value.bytes.to_vec().try_into() {
                    Ok(v) => v,
                    Err(_) => return Ok((name + "_axis", BattlebitType::Axis(Axis(0)))),
                });
                Ok((name + "_axis", BattlebitType::Axis(Axis(value as i32))))
            }
            "key" => {
                let value = i32::from_le_bytes(match value.bytes.to_vec().try_into() {
                    Ok(v) => v,
                    Err(_) => return Ok((name + "_key", BattlebitType::Key(Key(0)))),
                });
                Ok((name + "_key", BattlebitType::Key(Key(value as i32))))
            }
            "r" | "g" | "b" | "a" => {
                let kv = match typ {
                    "r" => 0,
                    "g" => 1,
                    "b" => 2,
                    "a" => 3,
                    _ => unreachable!("Invalid color type"),
                };

                let value = f64::from_le_bytes(match value.bytes.to_vec().try_into() {
                    Ok(v) => v,
                    Err(_) => return Ok((name + "_" + typ, BattlebitType::Int(Int(0)))),
                });
                Ok((
                    name + "_" + typ,
                    BattlebitType::Color(Color(kv, value as f64)),
                ))
            }
            _ => {
                let value = match String::from_utf8(value.bytes.to_vec()) {
                    Ok(v) => v,
                    Err(_) => String::from(""),
                };
                Ok((name, BattlebitType::Str(Str(value))))
            }
        }
    }

    pub fn to_toml_value(&self) -> Value {
        match self {
            BattlebitType::Int(i) => Value::Integer(i.0 as i64),
            BattlebitType::Float(f) => Value::Float(f.0),
            BattlebitType::Bool(b) => Value::Boolean(b.0),
            BattlebitType::Axis(a) => Value::Integer(a.0 as i64),
            BattlebitType::Color(c) => Value::Float(c.1 as f64),
            BattlebitType::Key(k) => Value::String(k.to_key_ascii()),
            BattlebitType::Str(s) => Value::String(s.0.clone()),
        }
    }

    pub fn to_reg_type_str(&self) -> &str {
        match self {
            BattlebitType::Int(_) => "int",
            BattlebitType::Float(_) => "float",
            BattlebitType::Bool(_) => "bool",
            BattlebitType::Axis(_) => "axis",
            BattlebitType::Color(_) => "color",
            BattlebitType::Key(_) => "key",
            BattlebitType::Str(_) => "str",
        }
    }

    pub fn to_reg_value(&self) -> RegValue {
        match self {
            BattlebitType::Int(i) => RegValue {
                bytes: i.0.to_le_bytes().to_vec(),
                vtype: winreg::enums::RegType::REG_DWORD,
            },
            BattlebitType::Float(f) => RegValue {
                bytes: f.0.to_le_bytes().to_vec(),
                vtype: winreg::enums::RegType::REG_DWORD,
            },
            BattlebitType::Bool(b) => RegValue {
                bytes: [b.0 as u8].to_vec(),
                vtype: winreg::enums::RegType::REG_DWORD,
            },
            BattlebitType::Axis(a) => RegValue {
                bytes: a.0.to_le_bytes().to_vec(),
                vtype: winreg::enums::RegType::REG_DWORD,
            },
            BattlebitType::Color(c) => RegValue {
                bytes: c.1.to_le_bytes().to_vec(),
                vtype: winreg::enums::RegType::REG_DWORD,
            },
            BattlebitType::Key(k) => RegValue {
                bytes: k.0.to_le_bytes().to_vec(),
                vtype: winreg::enums::RegType::REG_DWORD,
            },
            BattlebitType::Str(s) => RegValue {
                bytes: s.0.as_bytes().to_vec(),
                vtype: winreg::enums::RegType::REG_BINARY,
            },
        }
    }
}

pub struct State {
    pub original_registry_keys: Vec<String>,
    pub registry: HashMap<String, BattlebitType>,
    pub updated_registry: HashMap<String, BattlebitType>,
}

impl State {
    pub fn new() -> std::io::Result<Self> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        let battlebit = hkcu.open_subkey(BBR_REG_SUBKEY)?;

        let registry = battlebit
            .enum_values()
            .filter_map(|x| {
                let (name, value) = match x {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!("Failed to enumerate registry value: {}", e);
                        return None;
                    }
                };

                if !name.is_empty() {
                    let value = match BattlebitType::from_reg(name, value) {
                        Ok(v) => v,
                        Err(e) => {
                            tracing::warn!("Failed to parse registry value: {}", e);
                            return None;
                        }
                    };
                    Some(value)
                } else {
                    None
                }
            })
            .collect();

        let state = State {
            original_registry_keys: battlebit
                .enum_values()
                .map(|x| x.expect("Failed to enumerate registry value").0)
                .collect(),
            registry,
            updated_registry: HashMap::new(),
        };

        Ok(state)
    }

    pub fn resolve_regedit_name(&self, name: String, typ: &BattlebitType) -> String {
        let mut name = match typ {
            BattlebitType::Color(_) => name,
            BattlebitType::Key(_) => name,
            BattlebitType::Axis(_) => name,
            _ => name + "_" + typ.to_reg_type_str(),
        };

        for value in &self.original_registry_keys {
            if value.contains(&name) {
                name = value.clone();
                break;
            }
        }
        name
    }

    pub fn update_registry(&mut self, name: &str, value: BattlebitType) {
        let name = self.resolve_regedit_name(name.to_owned(), &value);
        self.updated_registry.insert(name, value);
    }

    pub fn save_registry(&self) -> std::io::Result<()> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let battlebit = hkcu.open_subkey_with_flags(BBR_REG_SUBKEY, KEY_SET_VALUE)?;
        for (name, value) in &self.updated_registry {
            let reg_value = value.to_reg_value();
            battlebit.set_raw_value(name, &reg_value)?;
        }
        Ok(())
    }
}
