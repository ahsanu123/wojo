mod repository_trait;

use repository_trait::*;
use sea_orm::DbErr;

pub struct CharacteristicRepository;

impl CharacteristicRepositoryTrait for CharacteristicRepository {
    async fn get_characteristics() -> Result<(), DbErr> {
        todo!()
    }
}
