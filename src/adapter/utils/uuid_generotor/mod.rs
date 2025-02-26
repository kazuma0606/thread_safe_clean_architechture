use crate::domain::utils::id_generator_interaface::IdGeneratorInterface;
use uuid::Uuid;
pub struct UuidGenerator;

impl UuidGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl IdGeneratorInterface for UuidGenerator {
    fn generate(&self) -> Uuid {
        Uuid::new_v4()
    }
}
