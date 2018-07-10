use std::panic::RefUnwindSafe;
use std::collections::BTreeMap;
use serde_json;

use helpers;
use helpers::{PathExtractor, QueryStringExtractor};
use model::Session;
use controller::{Controller, ResponseType};

use data_access::notification::Notification as NotificationData;

#[derive(Clone)]
pub struct UpdateNotificationsController {
    notification_data: NotificationData
}

impl UpdateNotificationsController {
    pub fn new(notification_data: NotificationData) -> UpdateNotificationsController {
        UpdateNotificationsController {
            notification_data: notification_data
        }
    }

    pub fn update_push_sub(&self, user_id: u64, subscription: Option<serde_json::value::Value>) -> String {
        let sub = subscription.unwrap();
        let subs = sub.as_object().unwrap();

        self.notification_data.update_push_subscription(user_id, subs["subscription"].to_string());

        let mut payload = BTreeMap::new();
        payload.insert("success", true);

        serde_json::to_string(&payload).unwrap()
    }
}

impl Controller for UpdateNotificationsController {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        body: Option<String>,
        _path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>,
    ) -> ResponseType {
        if helpers::is_logged_in(session) {
            let id = helpers::get_user_id(session).expect("no user id") as u64;
            let unwrapped_body = body.expect("no body for notifications update");
            let parsed_body = serde_json::from_str(&unwrapped_body).expect("unable to parse body");
            let json = self.update_push_sub(id, Some(parsed_body));
            ResponseType::Json(json)
        } else {
           ResponseType::Redirect("/".to_string())
        }
    }
}

impl RefUnwindSafe for UpdateNotificationsController {}
