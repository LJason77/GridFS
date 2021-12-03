use serde::{Deserialize, Serialize};

/// 对象结果
///
/// 返回给前端的对象结果
#[derive(Default, Serialize, Deserialize)]
pub struct Results<'a, T> {
    /// 状态码
    ///
    /// 返回正常时为空，只在错误时出现
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    /// 内容
    ///
    /// 具体内容视情况而定
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// 信息
    ///
    /// 一般情况下为空，错误时出现
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<&'a str>,
}
