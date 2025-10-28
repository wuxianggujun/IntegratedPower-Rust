// Recognition Rule Trait and Implementations
use crate::models::RowData;

/// 识别规则的核心trait
/// 
/// 该trait定义了行类型识别规则的基本接口。实现此trait的类型可以用于判断
/// Excel行是否符合特定的识别条件。
/// 
/// # 示例
/// 
/// ```rust
/// use crate::engine::recognition_rule::RecognitionRule;
/// use crate::models::RowData;
/// 
/// struct MyCustomRule {
///     name: String,
/// }
/// 
/// impl RecognitionRule for MyCustomRule {
///     fn name(&self) -> &str {
///         &self.name
///     }
///     
///     fn matches(&self, row_data: &RowData) -> bool {
///         // 自定义匹配逻辑
///         true
///     }
///     
///     fn clone_box(&self) -> Box<dyn RecognitionRule> {
///         Box::new(MyCustomRule {
///             name: self.name.clone(),
///         })
///     }
/// }
/// ```
pub trait RecognitionRule: Send + Sync {
    /// 返回规则的名称
    /// 
    /// 规则名称用于调试和日志记录，应该是唯一且具有描述性的。
    fn name(&self) -> &str;
    
    /// 评估规则是否匹配给定的行
    /// 
    /// # Arguments
    /// 
    /// * `row_data` - 要评估的行数据，包含所有单元格信息
    /// 
    /// # Returns
    /// 
    /// * `true` - 规则匹配，该行符合此规则定义的条件
    /// * `false` - 规则不匹配，该行不符合此规则定义的条件
    fn matches(&self, row_data: &RowData) -> bool;
    
    /// 获取规则的置信度
    /// 
    /// 置信度表示规则匹配的可靠程度，范围为0.0到1.0。
    /// 默认实现返回1.0，表示完全确定。
    /// 
    /// # Returns
    /// 
    /// 置信度值，范围[0.0, 1.0]
    fn confidence(&self) -> f32 {
        1.0
    }
    
    /// 克隆规则到Box中
    /// 
    /// 由于trait对象不能直接克隆，此方法提供了一种克隆规则的方式。
    /// 实现者应该返回一个包含自身克隆的Box。
    /// 
    /// # Returns
    /// 
    /// 包含规则克隆的Box
    fn clone_box(&self) -> Box<dyn RecognitionRule>;
}

/// 为Box<dyn RecognitionRule>实现Clone
/// 
/// 这允许我们克隆包含trait对象的Box
impl Clone for Box<dyn RecognitionRule> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
