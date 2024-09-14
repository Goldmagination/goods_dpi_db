use std::collections::HashMap;

use crate::db::Pool;
use crate::models::booking_aggregate::{
    booking::Booking, booking_assignment::BookingAssignment, booking_status::BookingStatus,
};
use crate::models::chat_aggregate::message_assignment::NewMessageAssignment;
use crate::models::chat_aggregate::{
    chat::Chat, chat::ChatItem, chat::NewChat, message::Message, message::NewMessage,
};
use crate::models::dtos::chat_dto::ChatDTO;
use crate::models::dtos::message_dto::{MessageAssignmentDTO, MessageDTO};
use crate::schema::schema::{bookings, chat, message, message_assignments, professional_profiles};
use actix_web::{web, Error as ActixError};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

pub fn get_messages_for_chat(
    conn: &mut PgConnection,
    chat_id_val: i32,
    limit_val: i64,
    offset_val: i64,
) -> QueryResult<Vec<ChatItem>> {
    // Fetch messages
    let messages: Vec<Message> = message::table
        .filter(message::chat_id.eq(chat_id_val))
        .order(message::timestamp.desc())
        .limit(limit_val)
        .offset(offset_val)
        .load::<Message>(conn)?;

    let message_ids: Vec<i32> = messages.iter().map(|msg| msg.id).collect();

    let message_assignments: Vec<(i32, String)> = message_assignments::table
        .filter(message_assignments::message_id.eq_any(&message_ids))
        .select((
            message_assignments::message_id,
            message_assignments::image_url,
        ))
        .load(conn)?;

    let assignments_map: HashMap<i32, Vec<MessageAssignmentDTO>> = message_assignments
        .into_iter()
        .fold(HashMap::new(), |mut acc, (message_id, image_url)| {
            acc.entry(message_id)
                .or_insert_with(Vec::new)
                .push(MessageAssignmentDTO::to_dto(message_id, image_url));
            acc
        });

    let message_dtos: Vec<MessageDTO> = messages
        .into_iter()
        .map(|msg| {
            let assignments = assignments_map.get(&msg.id).cloned();

            MessageDTO::to_dto(
                msg.id,
                msg.chat_id,
                msg.sender_uid,
                msg.text,
                msg.timestamp,
                msg.is_read,
                msg.receiver_uid,
                assignments,
            )
        })
        .collect();

    let mut chat_items: Vec<ChatItem> = message_dtos.into_iter().map(ChatItem::Message).collect();

    let bookings: Vec<ChatItem> = bookings::table
        .filter(bookings::chat_id.eq(chat_id_val))
        .order(bookings::date_time.desc())
        .limit(limit_val)
        .offset(offset_val)
        .load::<Booking>(conn)?
        .into_iter()
        .map(ChatItem::Booking)
        .collect();

    chat_items.extend(bookings);
    chat_items.sort_by(|a, b| a.get_time().cmp(&b.get_time()).reverse());

    Ok(chat_items)
}

pub fn get_chats_for_user(conn: &mut PgConnection, user_uid: &str) -> QueryResult<Vec<ChatDTO>> {
    // Fetch all chat IDs for the user
    let chat_ids: Vec<i32> = chat::table
        .filter(chat::user_uid.eq(user_uid))
        .select(chat::id)
        .load(conn)?;

    // Fetch the latest message IDs per chat
    use diesel::dsl::max;
    let latest_message_ids_per_chat: Vec<(i32, Option<i32>)> = message::table
        .filter(message::chat_id.eq_any(&chat_ids))
        .group_by(message::chat_id)
        .select((message::chat_id, max(message::id)))
        .load(conn)?;

    let latest_message_ids: Vec<i32> = latest_message_ids_per_chat
        .iter()
        .filter_map(|(_, message_id)| *message_id)
        .collect();

    // Fetch the latest messages
    let latest_messages: Vec<(
        i32,            // id
        i32,            // chat_id
        String,         // sender_uid
        Option<String>, // text
        NaiveDateTime,  // timestamp
        bool,           // is_read
        String,         // receiver_uid
    )> = message::table
        .filter(message::id.eq_any(&latest_message_ids))
        .select((
            message::id,
            message::chat_id,
            message::sender_uid,
            message::text.nullable(),
            message::timestamp,
            message::is_read,
            message::receiver_uid,
        ))
        .load(conn)?;

    // Fetch message assignments for the latest messages
    let message_assignments: Vec<(i32, String)> = message_assignments::table
        .filter(message_assignments::message_id.eq_any(&latest_message_ids))
        .select((
            message_assignments::message_id,
            message_assignments::image_url,
        ))
        .load(conn)?;

    // Map assignments to their corresponding messages
    let message_assignments_map: HashMap<i32, Vec<MessageAssignmentDTO>> = message_assignments
        .into_iter()
        .fold(HashMap::new(), |mut acc, (message_id, image_url)| {
            acc.entry(message_id)
                .or_insert_with(Vec::new)
                .push(MessageAssignmentDTO::to_dto(message_id, image_url));
            acc
        });

    // Create a map of chat IDs to their latest messages
    let latest_messages_map: HashMap<i32, MessageDTO> = latest_messages
        .into_iter()
        .map(
            |(id, chat_id, sender_uid, text, timestamp, is_read, receiver_uid)| {
                let assignments = message_assignments_map.get(&id).cloned();
                let message_dto = MessageDTO::to_dto(
                    id,
                    chat_id,
                    sender_uid,
                    text,
                    timestamp,
                    is_read,
                    receiver_uid,
                    assignments,
                );
                (chat_id, message_dto)
            },
        )
        .collect();

    // Fetch chats along with professional profiles
    let chats_with_profiles: Vec<(Chat, String, Option<String>)> =
        chat::table
            .inner_join(professional_profiles::table.on(
                chat::professional_profile_uid.eq(professional_profiles::professional_profile_uid),
            ))
            .filter(chat::user_uid.eq(user_uid))
            .select((
                chat::all_columns,
                professional_profiles::professional_name,
                professional_profiles::image_url,
            ))
            .load(conn)?;

    // Build ChatDTOs by associating each chat with its latest message
    let chat_dtos = chats_with_profiles
        .into_iter()
        .map(|(chat, professional_name, image_url)| {
            let message = latest_messages_map.get(&chat.id).cloned();
            let messages = message.map(|msg| vec![msg]); // Wrap message in a vector if it exists
            ChatDTO::chat_to_dto(chat, professional_name, image_url, messages)
        })
        .collect();

    Ok(chat_dtos)
}

pub async fn send_message(
    pool: web::Data<Pool>,
    sender_id: String,
    receiver_id: String,
    text: Option<String>,
    image_urls: Option<Vec<String>>,
) -> Result<i32, ActixError> {
    let mut conn = pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Could not get db connection: {}", e))
    })?;

    let message_id: i32 = web::block(move || -> Result<i32, diesel::result::Error> {
        conn.transaction(|conn| {
            let chat_id = get_or_create_chat(conn, &sender_id, &receiver_id)?;

            let new_message = NewMessage::create_message(
                chat_id,
                receiver_id.clone(),
                sender_id.clone(),
                text.clone(),
            );

            let inserted_message_id = diesel::insert_into(message::table)
                .values(&new_message)
                .returning(message::id)
                .get_result::<i32>(conn)?;

            if let Some(urls) = image_urls {
                let new_assignments: Vec<NewMessageAssignment> = urls
                    .into_iter()
                    .map(|url| NewMessageAssignment {
                        message_id: inserted_message_id,
                        image_url: url,
                    })
                    .collect();

                diesel::insert_into(message_assignments::table)
                    .values(&new_assignments)
                    .execute(conn)?;
            }

            diesel::update(chat::table.find(chat_id))
                .set(chat::last_message_time.eq(new_message.timestamp))
                .execute(conn)?;

            Ok(inserted_message_id)
        })
    })
    .await?
    .map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Error saving message: {}", e))
    })?;

    Ok(message_id)
}

pub fn read_message(conn: &mut PgConnection, message_id: &i32) -> QueryResult<Message> {
    diesel::update(message::table.filter(message::id.eq(message_id)))
        .set(message::is_read.eq(true))
        .get_result(conn)
}
pub fn retrieve_chat(
    conn: &mut PgConnection,
    user_uid: &str,
    professional_profile_uid: &str,
) -> QueryResult<Option<ChatDTO>> {
    // Find the chat between the two users
    let chat = chat::table
        .filter(
            chat::user_uid
                .eq(user_uid)
                .and(chat::professional_profile_uid.eq(professional_profile_uid)),
        )
        .or_filter(
            chat::user_uid
                .eq(professional_profile_uid)
                .and(chat::professional_profile_uid.eq(user_uid)),
        )
        .first::<Chat>(conn)
        .optional()?;

    let chat = match chat {
        Some(c) => c,
        None => return Ok(None),
    };

    // Fetch the latest message in the chat
    let latest_message: Option<Message> = message::table
        .filter(message::chat_id.eq(chat.id))
        .order(message::timestamp.desc())
        .first(conn)
        .optional()?;

    if let Some(msg) = latest_message {
        // Fetch all assignments for the latest message
        let message_assignments: Vec<(i32, String)> = message_assignments::table
            .filter(message_assignments::message_id.eq(msg.id))
            .select((
                message_assignments::message_id,
                message_assignments::image_url,
            ))
            .load(conn)?;

        // Map assignments to MessageAssignmentDTO
        let assignments = if !message_assignments.is_empty() {
            Some(
                message_assignments
                    .into_iter()
                    .map(|(message_id, image_url)| {
                        MessageAssignmentDTO::to_dto(message_id, image_url)
                    })
                    .collect::<Vec<MessageAssignmentDTO>>(),
            )
        } else {
            None
        };

        // Create MessageDTO
        let message_dto = MessageDTO::to_dto(
            msg.id,
            msg.chat_id,
            msg.sender_uid,
            msg.text,
            msg.timestamp,
            msg.is_read,
            msg.receiver_uid,
            assignments,
        );

        // Fetch professional profile info
        let professional_profile = professional_profiles::table
            .filter(
                professional_profiles::professional_profile_uid.eq(&chat.professional_profile_uid),
            )
            .select((
                professional_profiles::professional_name,
                professional_profiles::image_url,
            ))
            .first::<(String, Option<String>)>(conn)
            .optional()?;

        let (professional_name, image_url) = professional_profile.unwrap_or((String::new(), None));

        Ok(Some(ChatDTO::chat_to_dto(
            chat,
            professional_name,
            image_url,
            Some(vec![message_dto]),
        )))
    } else {
        // No messages in the chat yet
        let professional_profile = professional_profiles::table
            .filter(
                professional_profiles::professional_profile_uid.eq(&chat.professional_profile_uid),
            )
            .select((
                professional_profiles::professional_name,
                professional_profiles::image_url,
            ))
            .first::<(String, Option<String>)>(conn)
            .optional()?;

        let (professional_name, image_url) = professional_profile.unwrap_or((String::new(), None));

        Ok(Some(ChatDTO::chat_to_dto(
            chat,
            professional_name,
            image_url,
            None,
        )))
    }
}

pub fn get_or_create_chat(
    conn: &mut PgConnection,
    user_uid: &str,
    professional_profile_uid: &str,
) -> QueryResult<i32> {
    let existing_chat = chat::table
        .filter(
            chat::user_uid
                .eq(user_uid)
                .and(chat::professional_profile_uid.eq(professional_profile_uid)),
        )
        .or_filter(
            chat::user_uid
                .eq(professional_profile_uid)
                .and(chat::professional_profile_uid.eq(user_uid)),
        )
        .select(chat::id)
        .first::<i32>(conn)
        .optional()?;

    match existing_chat {
        Some(id) => Ok(id),
        None => {
            let new_chat = NewChat {
                user_uid: user_uid.to_string(),
                professional_profile_uid: professional_profile_uid.to_string(),
                last_message_time: Utc::now().naive_utc(),
            };

            diesel::insert_into(chat::table)
                .values(&new_chat)
                .returning(chat::id)
                .get_result::<i32>(conn)
        }
    }
}
