use sea_orm::{entity::prelude::*, Set, Database};



mod pfp_user_sync {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "pfp_user_sync")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: i64,
        #[sea_orm(primary_key, auto_increment = false)]
        pub guild_id: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            panic!("No RelationDef")
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub async fn test_db_instance() -> DatabaseConnection {
    let url = "postgres://postgres:postgres@localhost:5432/ci_db_test";
    // Connecting SQLite
    let db = Database::connect(url).await.unwrap();
    pfp_user_sync::Entity::delete_many().exec(&db).await.unwrap();

    db
}

async fn do_thing(db: &DatabaseConnection) {
    let exists = pfp_user_sync::Entity::find()
        .filter(
            pfp_user_sync::Column::UserId
                .eq(0)
                .and(pfp_user_sync::Column::GuildId.eq(0)),
        )
        .one(db)
        .await;

    let exists = match exists {
        Ok(exists) => exists.is_some(),
        Err(_) => false,
    };
    if !exists {
        pfp_user_sync::Entity::insert(pfp_user_sync::ActiveModel {
            user_id: Set(0),
            guild_id: Set(0),
        }).exec(db).await;
    }
}


#[tokio::main]
async fn main() {
    let db = test_db_instance().await;
    do_thing(&db).await;
    do_thing(&db).await;

    let sync = pfp_user_sync::Entity::find()
        .filter(
            pfp_user_sync::Column::UserId
                .eq(0)
                .and(pfp_user_sync::Column::GuildId.eq(0)),
        )
        .one(&db)
        .await
        .unwrap();
    assert!(sync.is_some());
}
