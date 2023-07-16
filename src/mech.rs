use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Mech {
    name: String,
    model: String,
    speed: u32,
    weight: u32,
}

impl Mech {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn weight(&self) -> u32 {
        self.weight
    }

    pub fn builder() -> MechBuilder {
        MechBuilder::new()
    }
}

#[derive(Default)]
pub struct MechBuilder {
    name: Option<String>,
    model: Option<String>,
    speed: Option<u32>,
    weight: Option<u32>,
}

impl MechBuilder {
    fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn speed(mut self, speed: u32) -> Self {
        self.speed = Some(speed);
        self
    }

    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn build(self) -> Result<Mech, MechBuilderError> {
        let name = self.name.ok_or(MechBuilderError::MissingName)?;
        let model = self.model.ok_or(MechBuilderError::MissingModel)?;
        let speed = self.speed.ok_or(MechBuilderError::MissingSpeed)?;
        let weight = self.weight.ok_or(MechBuilderError::MissingWeight)?;
        Ok(Mech {
            name,
            model,
            speed,
            weight,
        })
    }
}

#[derive(Debug, Error)]
pub enum MechBuilderError {
    #[error("Missing name")]
    MissingName,
    #[error("Missing model")]
    MissingModel,
    #[error("Missing speed")]
    MissingSpeed,
    #[error("Missing weight")]
    MissingWeight,
}