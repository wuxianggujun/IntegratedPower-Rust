/// 应用程序状态（保留用于向后兼容）
#[derive(Debug, Clone)]
pub enum AppState {
    /// 空闲状态
    Idle,
    /// 处理中状态
    Processing {
        /// 处理器 ID
        processor_id: String,
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
    pub fn processing(processor_id: String) -> Self {
        Self::Processing { processor_id }
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
            Self::Processing { processor_id } => Some(processor_id),
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

/// 应用程序视图
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    /// 主页 - 功能选择
    Home,
    /// 处理中
    Processing,
    /// 设置
    Settings,
    /// 历史记录
    History,
}

/// 处理状态（用于 egui 应用）
#[derive(Debug)]
pub enum ProcessingState {
    /// 空闲状态
    Idle,
    /// 处理中状态
    Processing {
        /// 取消发送通道
        cancel_tx: std::sync::mpsc::Sender<()>,
    },
    /// 完成状态
    Completed(crate::models::ProcessingResult),
    /// 错误状态
    Error(String),
}
