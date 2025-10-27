use crate::config::manager::ConfigManager;
use crate::error::AppError;
use crate::history::manager::HistoryManager;
use crate::processor::manager::ProcessorManager;

/// 主应用程序结构
pub struct Application {
    config_manager: ConfigManager,
    processor_manager: ProcessorManager,
    history_manager: HistoryManager,
}

impl Application {
    /// 创建新的应用程序实例
    pub fn new() -> Result<Self, AppError> {
        tracing::info!("初始化 IntegratedPower 应用程序");

        // 加载配置
        let config_manager = ConfigManager::load()?;

        // 创建处理器管理器
        let processor_manager = ProcessorManager::new();

        // 创建历史记录管理器
        let history_manager = HistoryManager::new(config_manager.config().max_history_entries)?;

        Ok(Self {
            config_manager,
            processor_manager,
            history_manager,
        })
    }

    /// 运行应用程序
    pub fn run(&mut self) -> Result<(), AppError> {
        tracing::info!("启动应用程序");

        // 注册示例处理器
        use crate::processor::examples::{DataCleaningProcessor, DataStatisticsProcessor};
        use std::sync::Arc;
        
        self.processor_manager.register_processor(Arc::new(DataCleaningProcessor::new()));
        self.processor_manager.register_processor(Arc::new(DataStatisticsProcessor::new()));

        println!("\n=== IntegratedPower ===");
        println!("版本: 0.1.0");
        println!("已注册 {} 个处理器", self.processor_manager.processor_count());
        println!("\n注意: Qt UI 尚未完全集成");
        println!("当前为命令行模式演示");
        println!("\n可用的处理器:");
        
        let processors = self.processor_manager.list_processors();
        for (i, proc) in processors.iter().enumerate() {
            println!("  {}. {} - {}", i + 1, proc.name, proc.description);
        }
        
        println!("\n配置信息:");
        println!("  主题: {:?}", self.config_manager.config().theme);
        println!("  最大历史记录: {}", self.config_manager.config().max_history_entries);
        println!("  并行处理: {}", self.config_manager.config().parallel_processing);
        println!("  最大并行任务: {}", self.config_manager.config().max_parallel_tasks);
        
        println!("\n历史记录:");
        println!("  当前记录数: {}", self.history_manager.entry_count());
        
        println!("\n提示: 完整的 Qt UI 需要以下步骤:");
        println!("  1. 确保已安装 Qt 6.10.0");
        println!("  2. 设置 CMAKE_PREFIX_PATH 环境变量");
        println!("  3. 实现 CXX-Qt 桥接代码");
        println!("  4. 编译并运行");
        
        println!("\n按 Enter 键退出...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();

        Ok(())
    }
}
