use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnCall {
    #[serde(rename = "_id")]
    pub id: String,
    pub month: u8,
    pub year: u16,
    pub user_id: String,
    pub on_call_group: String,
    pub days: Option<Vec<OnCallDay>>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct OnCallDay {
    pub day: u8,
    pub vacation_day: bool,
    pub oncall_type: Option<OnCallType>,
    pub oncall_duration: Option<f32>,
    pub sum: Option<f32>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OnCallType {
    NormalDay,
    Saturday12h,
    Saturday24h,
    Sunday12h,
    Sunday24h,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnCallMonth {
    #[serde(rename = "_id")]
    pub id: String,
    pub month: u8
}