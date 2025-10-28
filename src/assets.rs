//! 资源内嵌辅助（图标）
//! 
//! - 默认不内嵌，避免体积增大；
//! - 启用 feature `embed_icons` 后，将在编译期内嵌 resources/icons 下的文件；
//! - 未启用时，返回 None，调用方可使用表情符号或磁盘文件作为回退。

/// 获取内嵌图标的二进制数据（PNG/SVG/ICO 等）。
/// name 为逻辑名称（示例："search"、"gear"、"analyze"）。
pub fn embedded_icon(name: &str) -> Option<&'static [u8]> {
    #[cfg(feature = "embed_icons")]
    {
        match name {
            // 示例：请在启用 feature 时，确保下列文件存在；
            // "search" => Some(include_bytes!("../resources/icons/search.png")),
            // "gear" => Some(include_bytes!("../resources/icons/gear.png")),
            // "analyze" => Some(include_bytes!("../resources/icons/analyze.png")),
            _ => None,
        }
    }

    #[cfg(not(feature = "embed_icons"))]
    {
        let _ = name; // 避免未使用警告
        None
    }
}

