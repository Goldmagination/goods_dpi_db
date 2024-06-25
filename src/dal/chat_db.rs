use crate::models::chat_aggregate::{
    chat::Chat, chat::ChatDTO, chat::NewChat, message::Message, message::NewMessage,
};
use crate::schema::schema::{chat, message, professional_profiles};
use chrono::Utc;
use diesel::dsl::*;
use diesel::pg::expression::dsl::any;
use diesel::prelude::*;
use diesel::result::Error;

pub fn get_chats_for_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<Vec<ChatDTO>> {
    // First, get all chat IDs for the user
    let chat_ids: Vec<i32> = chat::table
        .filter(chat::user_id.eq(user_id))
        .select(chat::id)
        .load(conn)?;

    // Get the latest message for each chat
    let latest_messages = message::table
        .filter(message::chat_id.eq(any(chat_ids.clone())))
        .group_by(message::chat_id)
        .select((
            message::chat_id,
            max(message::timestamp),
            max(message::text),
        ))
        .load::<(i32, Option<chrono::NaiveDateTime>, Option<String>)>(conn)?;

    // Create a HashMap for quick lookup of latest messages
    let latest_messages_map: std::collections::HashMap<
        i32,
        (Option<chrono::NaiveDateTime>, Option<String>),
    > = latest_messages
        .into_iter()
        .map(|(chat_id, timestamp, text)| (chat_id, (timestamp, text)))
        .collect();

    // Now get all chats with their associated professional profiles
    let chats_with_profiles = chat::table
        .inner_join(professional_profiles::table)
        .filter(chat::user_id.eq(user_id))
        .select((
            chat::all_columns,
            professional_profiles::professional_name,
            professional_profiles::image_url,
        ))
        .load::<(Chat, String, Option<String>)>(conn)?;

    // Combine all the data into ChatDTOs
    let chat_dtos = chats_with_profiles
        .into_iter()
        .map(|(chat, professional_name, image_url)| {
            let (_, last_message) = latest_messages_map
                .get(&chat.id)
                .cloned()
                .unwrap_or((None, None));
            ChatDTO {
                id: chat.id,
                user_id: chat.user_id,
                professional_profile_id: chat.professional_profile_id,
                professional_name,
                image_url,
                last_message: last_message.unwrap_or_default(),
            }
        })
        .collect();

    Ok(chat_dtos)
}

/*
pub fn get_chats_for_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<Vec<ChatDTO>> {
    sql_function!(fn max(x: Timestamp) -> Nullable<Timestamp>);
    let latest_messages_cte = diesel::sql_query(
         "WITH latest_messages AS (
             SELECT
                 chat_id,
                 MAX(timestamp) as latest_timestamp
             FROM
                 message
             GROUP BY
                 chat_id
         )
         SELECT
             chat.*,
             professional_profiles.professional_name,
             professional_profiles.image_url,
             message.text
         FROM
             chat
         INNER JOIN
             professional_profiles ON chat.professional_profile_id = professional_profiles.id
         LEFT JOIN
             latest_messages ON chat.id = latest_messages.chat_id
         LEFT JOIN
             message ON chat.id = message.chat_id AND latest_messages.latest_timestamp = message.timestamp
         WHERE
             chat.user_id = $1"
     ).bind::<Integer, _>(user_id);

    let chats_with_details =
        latest_messages_cte.load::<(Chat, String, Option<String>, Option<String>)>(conn)?;

    let chat_dtos = chats_with_details
        .into_iter()
        .map(|(chat, title, image_url, last_message)| {
            ChatDTO::chat_to_dto(&chat, title, last_message.unwrap_or_default(), image_url)
        })
        .collect();

    Ok(chat_dtos)
}
// */
/*
// Method to retrieve the list of chats for a user
pub fn get_chats_for_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<Vec<Chat>> {
    chat::table
        .filter(chat::user_id.eq(user_id))
        .load::<Chat>(conn)
}
// */
pub fn get_messages_for_chat(conn: &mut PgConnection, chat_id: i32) -> QueryResult<Vec<Message>> {
    message::table
        .filter(message::chat_id.eq(chat_id))
        .load::<Message>(conn)
}

pub fn get_messages_for_sender_and_receiver(
    conn: &mut PgConnection,
    user_id: i32,
    professional_profile_id: i32,
) -> QueryResult<Vec<Message>> {
    let chat_id = chat::table
        .filter(
            chat::user_id
                .eq(user_id)
                .and(chat::professional_profile_id.eq(professional_profile_id)),
        )
        .or_filter(
            chat::user_id
                .eq(professional_profile_id)
                .and(chat::professional_profile_id.eq(user_id)),
        )
        .select(chat::id)
        .first::<i32>(conn);

    match chat_id {
        Ok(chat_id) => {
            // If chat ID is found, fetch messages for that chat
            message::table
                .filter(message::chat_id.eq(chat_id))
                .load::<Message>(conn)
        }
        Err(_) => {
            // If chat ID is not found, return empty vector (no messages)
            Ok(vec![])
        }
    }
}

pub async fn send_message(
    conn: &mut PgConnection,
    sender_id: i32,
    receiver_id: i32,
    text: &str,
) -> Result<(), Error> {
    // Check if a chat between sender_id and receiver_id exists
    let existing_chat_id = chat::table
        .filter(
            chat::user_id
                .eq(sender_id)
                .and(chat::professional_profile_id.eq(receiver_id)),
        )
        .or_filter(
            chat::user_id
                .eq(receiver_id)
                .and(chat::professional_profile_id.eq(sender_id)),
        )
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

async fn create_new_chat(
    conn: &mut PgConnection,
    user_id: i32,
    professional_profile_id: i32,
) -> Result<i32, Error> {
    let new_chat = NewChat {
        user_id,
        professional_profile_id,
        last_message_time: Utc::now().naive_utc(),
    };

    let chat_id = diesel::insert_into(chat::table)
        .values(&new_chat)
        .returning(chat::id)
        .get_result::<i32>(conn)
        .expect("Error inserting new chat");

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
