use std::collections::HashMap;

use crate::db::Pool;
use crate::models::booking_aggregate::{
    booking::Booking, booking_assignment::BookingAssignment, booking_status::BookingStatus,
};
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
    let messages: Vec<ChatItem> = message::table
        .filter(message::chat_id.eq(chat_id_val))
        .order(message::timestamp.desc())
        .limit(limit_val)
        .offset(offset_val)
        .load::<Message>(conn)?
        .into_iter()
        .map(ChatItem::Message)
        .collect();

    let bookings: Vec<ChatItem> = bookings::table
        .filter(bookings::chat_id.eq(chat_id_val))
        .order(bookings::date_time.desc())
        .limit(limit_val)
        .offset(offset_val)
        .load::<Booking>(conn)?
        .into_iter()
        .map(ChatItem::Booking)
        .collect();

    let mut chat_items = Vec::new();
    chat_items.extend(messages);
    chat_items.extend(bookings);
    chat_items.sort_by(|a, b| a.get_time().cmp(&b.get_time()).reverse());

    Ok(chat_items)
}

pub fn get_chats_for_user(conn: &mut PgConnection, user_uid: &str) -> QueryResult<Vec<ChatDTO>> {
    let chat_uids: Vec<i32> = chat::table
        .filter(chat::user_uid.eq(user_uid))
        .select(chat::id)
        .load(conn)?;

    let latest_messages: Vec<(i32, i32, String, String, NaiveDateTime, bool, String)> =
        message::table
            .filter(message::chat_id.eq_any(&chat_uids))
            .order((message::chat_id, message::timestamp.desc()))
            .select((
                message::id,
                message::chat_id,
                message::sender_uid,
                message::text,
                message::timestamp,
                message::is_read,
                message::receiver_uid,
            ))
            .load(conn)?;

    let message_ids: Vec<i32> = latest_messages.iter().map(|msg| msg.0).collect();
    let message_assignments: Vec<(i32, String)> = message_assignments::table
        .filter(message_assignments::message_id.eq_any(&message_ids))
        .select((
            message_assignments::message_id,
            message_assignments::image_url,
        ))
        .load(conn)?;

    let message_assignments_map: HashMap<i32, MessageAssignmentDTO> = message_assignments
        .into_iter()
        .map(|(message_id, image_url)| {
            (
                message_id,
                MessageAssignmentDTO::to_dto(message_id, image_url),
            )
        })
        .collect();

    let latest_messages_map: HashMap<i32, Vec<MessageDTO>> = latest_messages.into_iter().fold(
        HashMap::new(),
        |mut acc, (id, chat_id, sender_id, text, timestamp, is_read, receiver_id)| {
            let assignment = message_assignments_map.get(&id).cloned();
            acc.entry(chat_id)
                .or_insert_with(Vec::new)
                .push(MessageDTO::to_dto(
                    id,
                    chat_id,
                    sender_id,
                    text,
                    timestamp,
                    is_read,
                    receiver_id,
                    assignment,
                ));
            acc
        },
    );

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

    let chat_dtos = chats_with_profiles
        .into_iter()
        .map(|(chat, professional_name, image_url)| {
            let messages = latest_messages_map.get(&chat.id).cloned();
            ChatDTO::chat_to_dto(chat, professional_name, image_url, messages)
        })
        .collect();

    Ok(chat_dtos)
}

pub async fn send_message(
    pool: web::Data<Pool>,
    sender_id: String,
    receiver_id: String,
    text: String,
) -> Result<i32, ActixError> {
    let mut conn = pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Could not get db connection: {}", e))
    })?;

    let message_id: i32 = web::block(move || -> Result<i32, diesel::result::Error> {
        conn.transaction(|conn| {
            let chat_id = get_or_create_chat(conn, &sender_id, &receiver_id)?;

            let new_message = NewMessage::create_message(chat_id, receiver_id, sender_id, text);

            let inserted_message_id = diesel::insert_into(message::table)
                .values(&new_message)
                .returning(message::id)
                .get_result::<i32>(conn)?;

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

    let latest_message: Option<(i32, i32, String, String, NaiveDateTime, bool, String)> =
        message::table
            .filter(message::chat_id.eq(chat.id))
            .order(message::timestamp.desc())
            .select((
                message::id,
                message::chat_id,
                message::sender_uid,
                message::text,
                message::timestamp,
                message::is_read,
                message::receiver_uid,
            ))
            .first(conn)
            .optional()?;

    let latest_message = match latest_message {
        Some(msg) => msg,
        None => {
            // No messages for this chat, return the chat DTO without messages
            return Ok(Some(ChatDTO::chat_to_dto(chat, String::new(), None, None)));
        }
    };

    let message_assignment: Option<(i32, String)> = message_assignments::table
        .filter(message_assignments::message_id.eq(latest_message.0)) // message_id
        .select((
            message_assignments::message_id,
            message_assignments::image_url,
        ))
        .first(conn)
        .optional()?;

    let assignment_dto = message_assignment
        .map(|(message_id, image_url)| MessageAssignmentDTO::to_dto(message_id, image_url));

    let message_dto = MessageDTO::to_dto(
        latest_message.0, // message id
        latest_message.1, // chat id
        latest_message.2, // sender_uid
        latest_message.3, // text
        latest_message.4, // timestamp
        latest_message.5, // is_read
        latest_message.6, // receiver_uid
        assignment_dto,   // assignment
    );

    let professional_profile = professional_profiles::table
        .filter(professional_profiles::professional_profile_uid.eq(&chat.professional_profile_uid))
        .select((
            professional_profiles::professional_name,
            professional_profiles::image_url,
        ))
        .first::<(String, Option<String>)>(conn)
        .optional()?;

    let (professional_name, image_url) = match professional_profile {
        Some((name, img_url)) => (name, img_url),
        None => (String::new(), None),
    };

    Ok(Some(ChatDTO::chat_to_dto(
        chat,
        professional_name,
        image_url,
        Some(vec![message_dto]),
    )))
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
