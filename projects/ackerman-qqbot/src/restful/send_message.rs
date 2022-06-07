use super::*;

use reqwest::Method;

use std::str::FromStr;
use toml::Value;
use url::Url;

/// `GET /channels/{channel_id}/messages/{message_id}`
///
/// <https://bot.q.qq.com/wiki/develop/api/openapi/message/get_message_of_id.html>
#[derive(Debug)]
pub struct GetMessageListResponse {
    pub items: Vec<MessageItem>,
}

impl GetMessageListResponse {
    pub fn end_point(key: &QQSecret) -> String {
        if cfg!(debug_assertions) {
            format!(
                "https://sandbox.api.sgroup.qq.com/channels/{channel_id}/messages",
                channel_id = key.channel_id(),
                //     message_id = 0
            )
        }
        else {
            format!(
                "https://api.sgroup.qq.com/channels/{channel_id}/messages",
                channel_id = key.channel_id(),
                //   message_id = 0
            )
        }
    }
    pub async fn send(key: &QQSecret) -> AckermanResult<Self> {
        let url = Url::from_str(&Self::end_point(key))?;
        let response = key.as_request(Method::GET, url).send().await?;
        if response.status().as_u16() > 300 {
            println!("{}", response.status().as_u16())
        }

        let value: Value = response.json().await?;
        println!("{:#?}", value);
        todo!();

        // Ok(Self { items: response.json().await? })
    }
}

#[derive(Deserialize, Debug)]
pub struct MessageItem {
    /// 频道名称
    pub name: String,
    /// 描述
    pub description: String,
    /// 频道头像地址
    #[serde(deserialize_with = "crate::utils::read_url")]
    pub icon: Url,
    /// 频道ID
    #[serde(deserialize_with = "crate::utils::read_u64")]
    pub id: u64,
    /// 	最大成员数
    pub max_members: u32,
    /// 成员数
    pub member_count: u32,
    /// 当前人是否是创建人
    pub owner: bool,
    /// 创建人用户ID
    #[serde(deserialize_with = "crate::utils::read_u64")]
    pub owner_id: u64,
    /// 加入时间
    pub joined_at: String,
}
