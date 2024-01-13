use crate::models::chat_aggregate::{chat::Chat, chat::NewChat, message::Message, message::NewMessage};
use crate::schema::schema::{chat, message};
use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;


// Method to retrieve the list of chats for a user
pub fn get_chats_for_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<Vec<Chat>> {
    chat::table
        .filter(chat::user_id.eq(user_id))
        .load::<Chat>(conn)
}

pub fn get_messages_for_chat(conn: &mut PgConnection, chat_id: i32) -> QueryResult<Vec<Message>> {
    message::table
        .filter(message::chat_id.eq(chat_id))
        .load::<Message>(conn)
}

pub fn get_messages_for_sender_and_receiver(conn: &mut PgConnection, user_id: i32, professional_profile_id: i32) -> QueryResult<Vec<Message>> {
    let chat_id= chat::table
    .filter(chat::user_id.eq(user_id).and(chat::professional_profile_id.eq(professional_profile_id)))
    .or_filter(chat::user_id.eq(professional_profile_id).and(chat::professional_profile_id.eq(user_id)))
    .select(chat::id)
    .first::<i32>(conn);

    match chat_id {
    Ok(chat_id) => {
        // If chat ID is found, fetch messages for that chat
        message::table
            .filter(message::chat_id.eq(chat_id))
            .load::<Message>(conn)
    },
    Err(_) => {
        // If chat ID is not found, return empty vector (no messages)
        Ok(vec![])
    }
}
}

pub async fn send_message(conn: &mut PgConnection, sender_id: i32, receiver_id: i32, text: &str) -> Result<(), Error> {
    // Check if a chat between sender_id and receiver_id exists
    let existing_chat_id = chat::table
        .filter(chat::user_id.eq(sender_id).and(chat::professional_profile_id.eq(receiver_id)))
        .or_filter(chat::user_id.eq(receiver_id).and(chat::professional_profile_id.eq(sender_id)))
        .select(chat::id)
        .first::<i32>(conn)
        .optional()?;

    let chat_id = match existing_chat_id {
        Some(id) => id,
        None => {
            // If no chat exists, create a new one and get its ID
            create_new_chat(conn, sender_id, receiver_id).await?
        }
    };
    println!("{:#?} ggg here we are getting a chat id", chat_id);
    // Create and save the new message
    let new_message = NewMessage {
        chat_id,
        sender_id,
        receiver_id,
        text: text.to_string(),
        timestamp: Utc::now().naive_utc(),
        is_read: false,
    };
    save_message_async(conn, new_message).await?;

    // Additional logic to notify the receiver or update the chat UI can be added here

    Ok(())
}



async fn create_new_chat(conn: &mut PgConnection, user_id: i32, professional_profile_id: i32) -> Result<i32, Error> {


    let new_chat = NewChat {
        user_id,
        professional_profile_id,
        last_message_time: Utc::now().naive_utc(),
    };


    let chat_id = diesel::insert_into(chat::table)
        .values(&new_chat)
        .returning(chat::id)
        .get_result::<i32>(conn).expect("Error inserting new chat");

    Ok(chat_id)
}




async fn save_message_async(conn: &mut PgConnection, new_message: NewMessage) -> Result<(), Error> {

    diesel::insert_into(message::table)
        .values(&new_message)
        .execute(conn)?;

    diesel::update(chat::table.filter(chat::id.eq(new_message.chat_id)))
        .set(chat::last_message_time.eq(new_message.timestamp))
        .execute(conn)?;
    Ok(())
}







