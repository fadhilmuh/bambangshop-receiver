use std::fmt::format;
use std::thread;

use rocket::http::Status;
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;

use bambangshop_receiver::{APP_CONFIG, REQWEST_CLIENT, Result, compose_error_response};
use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::repository::notification::NotificationRepository;

pub struct NotificationService;

impl NotificationService{
}