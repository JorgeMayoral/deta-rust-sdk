use thiserror::Error;

#[derive(Error, Debug)]
pub enum DetaError {
    #[error("Invalid project key")]
    InvalidProjectKey,
}

#[derive(Debug)]
pub struct Deta {
    project_id: String,
    project_key: String,
}

impl Deta {
    pub fn new(project_key: String) -> Result<Self, DetaError> {
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
        let project_key = "1234_abc".to_string();
        let deta = Deta::new(project_key);
        assert!(deta.is_ok());
        let deta = deta.unwrap();
        assert_eq!(deta.project_id, "1234");
        assert_eq!(deta.project_key, "1234_abc");
    }

    #[test]
    fn test_new_deta_with_incorrect_key() {
        let project_key = "1234".to_string();
        let deta = Deta::new(project_key);
        assert!(deta.is_err());
    }
}
