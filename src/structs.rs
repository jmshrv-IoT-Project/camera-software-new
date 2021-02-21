use serde::{Deserialize, Serialize};
use uuid::Error as UuidError;
use uuid::Uuid;

/// This struct holds all the camera software settings and is loaded on startup.
#[derive(Serialize, Deserialize, Debug)]
pub struct CameraSoftwareSettings {
    pub base_url: String,
    pub camera_token: Uuid,
}

impl CameraSoftwareSettings {
    pub fn from_qr_string(qr_string: String) -> Result<CameraSoftwareSettings, UuidError> {
        let qr_string_split: Vec<&str> = qr_string.split(",").collect();
        let camera_token_uuid = Uuid::parse_str(qr_string_split[1])?;

        Ok(CameraSoftwareSettings {
            base_url: qr_string_split[0].to_string(),
            camera_token: camera_token_uuid,
        })
    }

    pub fn load_from_confy() -> Result<CameraSoftwareSettings, confy::ConfyError> {
        confy::load("camera-software-new")
    }
}

impl Default for CameraSoftwareSettings {
    fn default() -> Self {
        CameraSoftwareSettings {
            base_url: "NOT_SET".to_string(),
            camera_token: Uuid::nil(),
        }
    }
}
