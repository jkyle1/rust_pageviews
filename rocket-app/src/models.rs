use chrono::NaiveDateTime;

use crate::schema::pageviews;

///represents pageview retrieved from db
#[derive(Serialize, Deserialize, Queryable)]
pub struct PageView {
    pub id: i64,
    pub view_time: NaiveDateTime,
    pub url: String,
    pub user_agent: String,
    pub referer: String,
    pub device_type: i8,
}

///represents pageview inserted into db
#[derive(Deserialize, Insertable)]
#[table_name = "pageviews"]
pub struct InsertablePageView {
    pub url: String,
    pub user_agent: String,
    pub referrer: String,
    pub device_type: i8,
}