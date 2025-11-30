use crate::ModName;
use custom_string::custom_string;

custom_string!(
    #[doc = "The mod path. (ex: the.mod.path)"],
    ModPath,
    ModPathRef,
    WithModPath,
    mod_path,
    |s| validate_mod_path(s)
);

/// Validates the `mod_path`.
pub fn validate_mod_path(mod_path: &str) -> Result<(), &'static str> {
    for mod_name in mod_path.split(".") {
        ModName::validate(mod_name)?;
    }
    Ok(())
}
