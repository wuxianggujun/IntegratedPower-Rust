// 示例处理器
pub mod example_processor1;
pub mod example_processor2;

#[allow(unused_imports)]
pub use example_processor1::DataCleaningProcessor;
#[allow(unused_imports)]
pub use example_processor2::DataStatisticsProcessor;
