// 3. Create new file src/controller/notification.rs, then insert these dependencies.
use rocket::response::status::Created;
use rocket::serde::json::Json;

use bambangshop::Result;
use crate::model::subscriber:: Subscriber;
use crate::service::notification:: NotificationService;
