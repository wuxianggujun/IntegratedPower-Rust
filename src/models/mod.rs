// Data Models 模块
pub mod progress;
pub mod result;
pub mod state;
pub mod processor_config;
pub mod row_type;

// 重新导出常用类型
pub use progress::ProcessingProgress;
pub use result::{ProcessingError, ProcessingResult, ProcessingStats};
pub use state::{AppState, AppView, ProcessingState};
pub use processor_config::{ProcessorConfig, ProcessorConfigs, InputType, ConfigValue};
pub use row_type::{
    RowType, RgbColor, CellStyle, MergeInfo, CellData, RowData, 
    WorksheetData, RowIdentificationResult, IdentificationStatistics,
};
