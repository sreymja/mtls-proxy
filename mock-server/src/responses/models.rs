use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<Model>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
}

pub struct ModelsResponseGenerator;

impl ModelsResponseGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_response(&self, available_models: &[String]) -> ModelsResponse {
        let mut models = Vec::new();
        
        for model_id in available_models {
            models.push(Model {
                id: model_id.clone(),
                object: "model".to_string(),
                created: 1698940800, // Fixed timestamp for consistency
                owned_by: "openai".to_string(),
            });
        }
        
        // Add some default models if none provided
        if models.is_empty() {
            models.extend_from_slice(&[
                Model {
                    id: "gpt-4o-mini".to_string(),
                    object: "model".to_string(),
                    created: 1698940800,
                    owned_by: "openai".to_string(),
                },
                Model {
                    id: "gpt-4o".to_string(),
                    object: "model".to_string(),
                    created: 1698940800,
                    owned_by: "openai".to_string(),
                },
                Model {
                    id: "gpt-3.5-turbo".to_string(),
                    object: "model".to_string(),
                    created: 1698940800,
                    owned_by: "openai".to_string(),
                },
            ]);
        }
        
        ModelsResponse {
            object: "list".to_string(),
            data: models,
        }
    }
}

impl Default for ModelsResponseGenerator {
    fn default() -> Self {
        Self::new()
    }
}
