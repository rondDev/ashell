use crate::app;

use super::{Module, OnModulePress};
use chrono::{DateTime, Local};
use iced::{Element, Subscription, time::every, widget::text};
use std::time::Duration;

pub struct Clock {
    date: DateTime<Local>,
}

impl Default for Clock {
    fn default() -> Self {
        Self { date: Local::now() }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Update,
}

impl Clock {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Update => {
                self.date = Local::now();
            }
        }
    }
}

impl Module for Clock {
    type ViewData<'a> = &'a str;
    type SubscriptionData<'a> = u64;
    fn view(
        &self,
        format: Self::ViewData<'_>,
    ) -> Option<(Element<app::Message>, Option<OnModulePress>)> {
        Some((text(self.date.format(format).to_string()).into(), None))
    }

    fn subscription(
        &self,
        update_time: Self::SubscriptionData<'_>,
    ) -> Option<Subscription<app::Message>> {
        Some(
            every(Duration::from_secs(update_time))
                .map(|_| Message::Update)
                .map(app::Message::Clock),
        )
    }
}
