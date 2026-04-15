mod repository_trait;

use repository_trait::*;
use sea_orm::DbErr;

pub struct ServiceRepository;

impl ServiceRepositoryTrait for ServiceRepository {
    async fn get_services() -> Result<(), DbErr> {
        todo!()
    }
}
