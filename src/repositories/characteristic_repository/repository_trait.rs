use sea_orm::DbErr;

pub trait CharacteristicRepositoryTrait {
    fn get_characteristics() -> impl Future<Output = Result<(), DbErr>>;
}
