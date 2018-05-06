use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;

use std::collections::BTreeMap;
use serde_json;

use helpers;
use bodyparser;

use data_access::notification::Notification as NotificationData;

#[derive(Clone)]
pub struct UpdateNotifications {
    notification_data: NotificationData
}

impl UpdateNotifications {
    pub fn new(notification_data: NotificationData) -> UpdateNotifications {
        UpdateNotifications {
            notification_data: notification_data
        }
    }

    pub fn update_push_sub(&self, user_id: u64, subscription: Option<serde_json::value::Value>) -> Response {
        let sub = subscription.unwrap();
        let subs = sub.as_object().unwrap();

        self.notification_data.update_push_subscription(user_id, subs["subscription"].to_string());

        let mut payload = BTreeMap::new();
        payload.insert("success", true);

        let success = json::encode(&payload).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Response::with((content_type, status::Ok, success))
    }

    fn get_body(&self, req: &mut Request) -> Option<serde_json::value::Value> {
        match req.get::<bodyparser::Json>(){
            Ok(json) => Some(json.expect("unable to unwrap json")),
            _ => None
        }
    }

    fn output_error(&self) -> Response {
        let mut error = BTreeMap::new();
        error.insert("error", true);

        let json_error = json::encode(&error).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Response::with((content_type, status::InternalServerError, json_error))

    }
}

impl Handler for UpdateNotifications {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref body = self.get_body(req);

        let session_user_id = helpers::get_user_id(req);

        let resp = match session_user_id {
            Some(user_id) => {
                info!("valid user - updating push subscription");
                self.update_push_sub(user_id, body.to_owned())
            },
            _ => {
                self.output_error()
            }
        };

        Ok(resp)
    }
}
