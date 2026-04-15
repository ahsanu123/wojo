use sea_orm::DbErr;

pub trait ServiceRepositoryTrait {
    fn get_services() -> impl Future<Output = Result<(), DbErr>>;
}
