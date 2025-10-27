use tokio_util::sync::CancellationToken;

/// 应用程序状态
#[derive(Debug, Clone)]
pub enum AppState {
    /// 空闲状态
    Idle,
    /// 处理中状态
    Processing {
        /// 处理器 ID
        processor_id: String,
        /// 取消令牌
        cancel_token: CancellationToken,
    },
    /// 错误状态
    Error(String),
}

impl AppState {
    /// 创建空闲状态
    pub fn idle() -> Self {
        Self::Idle
    }

    /// 创建处理中状态
    pub fn processing(processor_id: String, cancel_token: CancellationToken) -> Self {
        Self::Processing {
            processor_id,
            cancel_token,
        }
    }

    /// 创建错误状态
    pub fn error(message: String) -> Self {
        Self::Error(message)
    }

    /// 检查是否为空闲状态
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// 检查是否为处理中状态
    pub fn is_processing(&self) -> bool {
        matches!(self, Self::Processing { .. })
    }

    /// 检查是否为错误状态
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    /// 获取处理器 ID（如果在处理中）
    pub fn processor_id(&self) -> Option<&str> {
        match self {
            Self::Processing { processor_id, .. } => Some(processor_id),
            _ => None,
        }
    }

    /// 获取取消令牌（如果在处理中）
    pub fn cancel_token(&self) -> Option<&CancellationToken> {
        match self {
            Self::Processing { cancel_token, .. } => Some(cancel_token),
            _ => None,
        }
    }

    /// 获取错误消息（如果在错误状态）
    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error(msg) => Some(msg),
            _ => None,
        }
    }
}
