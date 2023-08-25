use crate::errors::DetaError;

const DETA_PROJECT_KEY_VAR_NAME: &str = "DETA_PROJECT_KEY";

#[derive(Debug)]
pub struct Deta {
    project_id: String,
    project_key: String,
}

impl Deta {
    pub fn new() -> Result<Self, DetaError> {
        let project_key =
            std::env::var(DETA_PROJECT_KEY_VAR_NAME).map_err(|_| DetaError::MissingProjectKey)?;
        let project_id = get_project_id(project_key.clone())?;
        Ok(Self {
            project_id,
            project_key,
        })
    }

    pub fn project_id(&self) -> &str {
        &self.project_id
    }

    pub fn project_key(&self) -> &str {
        &self.project_key
    }
}

impl TryFrom<String> for Deta {
    type Error = DetaError;

    fn try_from(project_key: String) -> Result<Self, Self::Error> {
        let project_id = get_project_id(project_key.clone())?;
        Ok(Self {
            project_id,
            project_key,
        })
    }
}

fn get_project_id(project_key: String) -> Result<String, DetaError> {
    let parts = project_key.split('_').collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Err(DetaError::InvalidProjectKey);
    }
    Ok(parts[0].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deta() {
        std::env::set_var(DETA_PROJECT_KEY_VAR_NAME, "1234_abc");
        let deta = Deta::new();
        assert!(deta.is_ok());
        let deta = deta.unwrap();
        assert_eq!(deta.project_id(), "1234");
        assert_eq!(deta.project_key(), "1234_abc");
    }

    #[test]
    fn test_new_deta_with_incorrect_key() {
        std::env::set_var(DETA_PROJECT_KEY_VAR_NAME, "1234");
        let deta = Deta::new();
        assert!(deta.is_err());
        assert_eq!(deta.unwrap_err(), DetaError::InvalidProjectKey);
    }

    #[test]
    fn test_new_deta_with_missing_key() {
        std::env::remove_var(DETA_PROJECT_KEY_VAR_NAME);
        let deta = Deta::new();
        assert!(deta.is_err());
        assert_eq!(deta.unwrap_err(), DetaError::MissingProjectKey);
    }

    #[test]
    fn test_new_deta_from_string() {
        let project_key = "1234_abc".to_string();
        let deta = Deta::try_from(project_key);
        assert!(deta.is_ok());
        let deta = deta.unwrap();
        assert_eq!(deta.project_id(), "1234");
        assert_eq!(deta.project_key(), "1234_abc");
    }

    #[test]
    fn test_new_deta_from_string_with_incorrect_key() {
        let project_key = "1234".to_string();
        let deta = Deta::try_from(project_key);
        assert!(deta.is_err());
        assert_eq!(deta.unwrap_err(), DetaError::InvalidProjectKey);
    }
}
