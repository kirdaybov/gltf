use std::fmt;

pub use serde_json::Value;

/// Data type of the `extras` attribute on all glTF objects.
#[cfg(feature = "extras")]
pub type Extras = Option<Value>;

/// Data type of the `extras` attribute on all glTF objects.
#[cfg(not(feature = "extras"))]
pub type Extras = Void;

/// Type representing no user-defined data.
#[derive(Clone, Default, Serialize, Deserialize, Validate)]
pub struct Void {
    #[serde(default, skip_serializing)]
    _allow_unknown_fields: (),
}

impl fmt::Debug for Void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}

impl fmt::Display for Void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}
