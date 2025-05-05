use crate::app;
use hyprland::event_listener::AsyncEventListener;
use iced::Task;
use iced::{Element, Subscription, stream::channel, widget::text};
use log::{debug, error};
use std::any::TypeId;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;

use super::{Module, OnModulePress};

#[derive(Default)]
pub struct SwayNc {
    count: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    SubscribeUpdate { count: i32 },
}

impl SwayNc {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::SubscribeUpdate { count } => {
                self.count = count;
                Task::none()
            }
        }
    }
}

impl Module for SwayNc {
    type ViewData<'a> = ();
    type SubscriptionData<'a> = ();

    fn view(
        &self,
        _: Self::ViewData<'_>,
    ) -> Option<(Element<app::Message>, Option<OnModulePress>)> {
        Some((
            text(self.count).into(),
            Some(OnModulePress::Action(app::Message::OpenSwayNc)),
        ))
    }

    fn subscription(&self, _: Self::SubscriptionData<'_>) -> Option<Subscription<app::Message>> {
        let id = TypeId::of::<Self>();

        Some(
            Subscription::run_with_id(
                id,
                channel(10, async |mut output| {
                    let stdout = Command::new("swaync-client")
                        .args(["--subscribe"])
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Couldn't spawn command for swaync-client")
                        .stdout
                        .expect("Error running command for swaync-client");
                    let reader = BufReader::new(stdout);

                    reader
                        .lines()
                        .filter_map(|line| {
                            let i = str::parse::<i32>(
                                serde_json::from_str(line.expect("Oops").as_str()).unwrap(),
                            );
                            Some(i)
                        })
                        .for_each(|line| {
                            let i: i32 = line.unwrap_or(0);
                            output
                                .try_send(Message::SubscribeUpdate { count: i })
                                .expect("oops")
                        });
                    let mut event_listener = AsyncEventListener::new();

                    debug!("Starting swaync-client");

                    let res = event_listener.start_listener_async().await;

                    if let Err(e) = res {
                        error!("restarting active window listener due to error: {:?}", e);
                    }
                }),
            )
            .map(app::Message::SubscribeUpdate),
        )
    }
}
