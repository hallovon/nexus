use crate::entity::messages;
use crate::entity::prelude::*;
use crate::Result;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use tracing::error;
use tracing::info;
use tracing::warn;

pub async fn create_message(
    db: &DatabaseConnection,
    sender_id: Uuid,
    receiver_id: Uuid,
    content: String,
) -> Result<()> {
    let new_message = messages::ActiveModel {
        sender_id: Set(sender_id),
        receiver_id: Set(receiver_id),
        content: Set(content),
        ..Default::default()
    };

    match Messages::insert(new_message).exec(db).await {
        Ok(insert_result) => {
            info!("Message inserted with ID: {}", insert_result.last_insert_id);
            Ok(())
        }
        Err(err) => {
            error!("Error inserting message: {:?}", err);
            Ok(())
        }
    }
}

async fn find_message(db: &DatabaseConnection, message_id: i32) -> Option<messages::Model> {
    match Messages::find_by_id(message_id).one(db).await {
        Ok(Some(message)) => Some(message),
        Ok(None) => {
            warn!("Message not found");
            None
        }
        Err(err) => {
            error!("Error querying message: {:?}", err);
            None
        }
    }
}

async fn find_user_messages(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<messages::Model>> {
    let messages = Messages::find()
        .filter(messages::Column::SenderId.eq(user_id))
        .all(db)
        .await?;
    Ok(messages)
}

async fn update_message_content(db: &DatabaseConnection, message_id: i32, new_content: String) {
    let message = Messages::find_by_id(message_id).one(db).await.unwrap();
    let mut message: messages::ActiveModel = message.unwrap().into();
    message.content = Set(new_content);
    message.update(db).await.unwrap();
}

async fn delete_message(db: &DatabaseConnection, message_id: i32) {
    match messages::Entity::delete_by_id(message_id).exec(db).await {
        Ok(_) => println!("Message deleted"),
        Err(err) => eprintln!("Error deleting message: {:?}", err),
    }
}
