// export interface IdGeneratorInterface {
//     generate(): string;
// }

use uuid::Uuid;

pub trait IdGeneratorInterface {
    fn generate(&self) -> Uuid;
}
