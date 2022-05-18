use super::*;
use toml::{value::Datetime, Value};
use url::Url;

/// `GET /users/@me/guilds`
///
/// <https://bot.q.qq.com/wiki/develop/api/openapi/user/guilds.html#%E8%8E%B7%E5%8F%96%E7%94%A8%E6%88%B7%E9%A2%91%E9%81%93%E5%88%97%E8%A1%A8>
#[derive(Serialize, Deserialize, Debug)]
pub struct GetGuildResponse {
    description: String,
    icon: Url,
    id: u64,
    joined_at: Datetime,
    max_members: u32,
    member_count: u32,
    name: String,
}

impl GetGuildResponse {
    pub async fn send(key: &SecretKey) -> AckermanResult<Self> {
        let request_url = if cfg!(debug_assertions) {
            format!("https://sandbox.api.sgroup.qq.com/users/@me/guilds")
        }
        else {
            format!("https://api.sgroup.qq.com/users/@me/guilds")
        };
        println!("{}", request_url);
        let response = Client::default()
            .get(request_url)
            .header(USER_AGENT, "BotNodeSDK/v2.9.4")
            .header(AUTHORIZATION, key.bot_token())
            .timeout(Duration::from_secs(3))
            .send()
            .await?;
        let out: Value = response.json().await?;
        println!("{}", out);
        todo!();
        // return Ok(out);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGuildItem {
    /// 子频道 id
    id: String,
    /// 频道 id
    guild_id: String,
    /// 子频道名
    name: String,
    /// 子频道类型 ChannelType
    r#type: u32,
    /// 子频道子类型 ChannelSubType
    sub_type: u32,
    /// 排序值，具体请参考 有关 position 的说明
    position: u32,
    /// 所属分组 id，仅对子频道有效，对 子频道分组（ChannelType=4） 无效
    parent_id: String,
    /// 创建人 id
    owner_id: String,
    /// 子频道私密类型 PrivateType
    private_type: u32,
    /// 子频道发言权限 SpeakPermission
    speak_permission: u32,
    /// 用于标识应用子频道应用类型，仅应用子频道时会使用该字段，具体定义请参考 应用子频道的应用类型
    application_id: String,
    /// 用户拥有的子频道权限 Permissions
    permissions: String,
}
