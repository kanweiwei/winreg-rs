#![deny(clippy::all)]
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_CLASSES_ROOT};
use winreg::RegKey;
use napi_derive::napi;

pub struct Registry {
  pub predef: RegKey
}

#[napi]
pub struct JsRegistry {
  registry: Registry
}

#[napi]
pub const HKLM: &str = "HKLM";

#[napi]
pub const HKCU: &str = "HKCU";

#[napi]
pub const HKCR: &str = "HKCR";

#[napi]
impl JsRegistry {
  #[napi(constructor)]
  pub fn new(hive: String) -> Self {
    let predef: RegKey;
    if hive == HKCU.to_string() {
      predef = RegKey::predef(HKEY_CURRENT_USER);
    } else if hive == "HKLM" {
      predef = RegKey::predef(HKEY_LOCAL_MACHINE);
    } else if hive == "HKCR" {
      predef = RegKey::predef(HKEY_CLASSES_ROOT);
    } else {
      predef = RegKey::predef(HKEY_CURRENT_USER);
    }
    JsRegistry {
      registry: Registry { predef },
    }
  }

  #[napi]
  pub fn get_key_value(&self, key: String, name: String) -> Option<String> {
    if let Ok(sub_key) = self.registry.predef.open_subkey(key) {
      match sub_key.get_value(name) {
        Ok(value) => Some(value),
        Err(_) => None,
      }
    } else {
      None
    }
  }

  #[napi]
  pub fn get_values(&self, key: String) -> Option<Vec<String>> {
    if let Ok(sub_key) = self.registry.predef.open_subkey(key) {
      let names: Vec<String> = sub_key.enum_values().map(|x| match x {
        Ok((x, _)) => {
            x.to_string()
        },
        Err(_err) => "".to_string(),
      })
      .filter(|s| !s.trim().is_empty())
      .collect();
      Some(names)
    } else {
      None
    }
  }

  #[napi]
  pub fn get_keys(&self, key: String) -> Option<Vec<String>> {
    if let Ok(sub_key) = self.registry.predef.open_subkey(key) {
      let names: Vec<String> = sub_key.enum_keys().map(|x| match x {
        Ok(x) => {
            x.to_string()
        },
        Err(_err) => "".to_string(),
      })
      .filter(|s| !s.trim().is_empty())
      .collect();
      Some(names)
    } else {
      None
    }
  }
}
