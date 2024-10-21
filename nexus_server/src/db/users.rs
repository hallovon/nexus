use crate::entity::{prelude::Users, users};
use crate::{DbPool, Result};
use sea_orm::prelude::Uuid;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::{ColumnTrait, EntityTrait};
use tracing::error;
use tracing::info;
use tracing::warn;

pub async fn create_user(db: DbPool, username: String, password_hash: String) -> Result<()> {
    let new_user = users::ActiveModel {
        username: Set(username),
        password_hash: Set(password_hash),
        ..Default::default()
    };

    match Users::insert(new_user).exec(&*db).await {
        Ok(insert_result) => info!("User inserted with ID: {}", insert_result.last_insert_id),
        Err(err) => error!("Error inserting user: {:?}", err),
    }

    Ok(())
}

pub async fn find_user(db: DbPool, user_id: Uuid) -> Option<users::Model> {
    let user = Users::find_by_id(user_id).one(&*db).await;
    match user {
        Ok(Some(user)) => Some(user),
        Ok(None) => {
            warn!("User not found, user_id: {:?}", user_id);
            None
        }
        Err(err) => {
            error!("Error querying user: {:?}", err);
            None
        }
    }
}

pub async fn find_user_by_username(db: DbPool, username: &str) -> Option<users::Model> {
    let user = Users::find()
        .filter(users::Column::Username.eq(username))
        .one(&*db)
        .await;

    match user {
        Ok(Some(user)) => Some(user),
        Ok(None) => None,
        Err(err) => {
            error!("Error querying user: {:?}", err);
            None
        }
    }
}

pub async fn update_user_username(db: DbPool, user_id: Uuid, new_username: String) -> Result<()> {
    let user = Users::find_by_id(user_id).one(&*db).await.unwrap();
    match user {
        Some(user) => {
            let mut user: users::ActiveModel = user.into();
            user.username = Set(new_username);
            user.update(&*db).await?;
            Ok(())
        }
        None => {
            warn!("User not found, user_id: {:?}", user_id);
            Ok(())
        }
    }
}

pub async fn delete_user(db: DbPool, user_id: Uuid) {
    let result = Users::delete_by_id(user_id).exec(&*db).await;
    match result {
        Ok(_) => println!("User deleted"),
        Err(err) => eprintln!("Error deleting user: {:?}", err),
    }
}
