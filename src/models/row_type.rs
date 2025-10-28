// Row Type Identification Models
use std::collections::HashMap;

/// 表示Excel行的语义类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RowType {
    /// 项目编号行 (浅绿色背景)
    ProjectNumber,
    /// 柜号行 (灰蓝色背景)
    CabinetNumber,
    /// 表头行 (浅灰色背景)
    Header,
    /// 数据行 (无背景色)
    Data,
    /// 小计行 (浅粉色背景)
    SubTotal,
    /// 单台合计行 (浅粉色背景)
    UnitTotal,
    /// 总计行 (浅粉色背景)
    GrandTotal,
    /// 未知类型
    Unknown,
    /// 自定义类型 (支持扩展)
    Custom(String),
}

/// RGB颜色值
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    /// 创建新的RGB颜色
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    /// 白色
    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }
}

/// 单元格样式信息
#[derive(Debug, Clone)]
pub struct CellStyle {
    /// 背景颜色
    pub background_color: Option<RgbColor>,
    /// 字体颜色
    pub font_color: Option<RgbColor>,
    /// 是否加粗
    pub bold: bool,
    /// 是否斜体
    pub italic: bool,
}

impl Default for CellStyle {
    fn default() -> Self {
        Self {
            background_color: None,
            font_color: None,
            bold: false,
            italic: false,
        }
    }
}

/// 单元格合并信息
#[derive(Debug, Clone)]
pub struct MergeInfo {
    /// 合并范围的起始列
    pub start_col: usize,
    /// 合并范围的结束列
    pub end_col: usize,
    /// 合并范围的起始行
    pub start_row: usize,
    /// 合并范围的结束行
    pub end_row: usize,
}

impl MergeInfo {
    /// 获取合并的列数
    pub fn column_span(&self) -> usize {
        self.end_col - self.start_col + 1
    }
    
    /// 获取合并的行数
    pub fn row_span(&self) -> usize {
        self.end_row - self.start_row + 1
    }
}

/// 表示Excel中单个单元格的数据
#[derive(Debug, Clone)]
pub struct CellData {
    /// 列索引 (从0开始)
    pub column_index: usize,
    /// 单元格内容
    pub content: String,
    /// 单元格样式
    pub style: CellStyle,
    /// 合并信息
    pub merge_info: Option<MergeInfo>,
}

impl CellData {
    /// 判断单元格是否为空
    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}

/// 表示Excel中一行的完整数据
#[derive(Debug, Clone)]
pub struct RowData {
    /// 行号 (从0开始)
    pub row_index: usize,
    /// 单元格数据列表
    pub cells: Vec<CellData>,
}

impl RowData {
    /// 获取指定列的单元格
    pub fn get_cell(&self, column_index: usize) -> Option<&CellData> {
        self.cells.get(column_index)
    }
    
    /// 获取第一个非空单元格
    pub fn first_non_empty_cell(&self) -> Option<&CellData> {
        self.cells.iter().find(|cell| !cell.is_empty())
    }
}

/// 表示Excel工作表的完整数据
#[derive(Debug, Clone)]
pub struct WorksheetData {
    /// 工作表名称
    pub name: String,
    /// 所有行数据
    pub rows: Vec<RowData>,
}

impl WorksheetData {
    /// 获取指定行
    pub fn get_row(&self, row_index: usize) -> Option<&RowData> {
        self.rows.get(row_index)
    }
    
    /// 获取总行数
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

/// 单行的识别结果
#[derive(Debug, Clone)]
pub struct RowIdentificationResult {
    /// 行号 (从0开始)
    pub row_index: usize,
    /// 识别出的行类型
    pub row_type: RowType,
    /// 匹配的规则名称
    pub matched_rule: String,
    /// 置信度 (0.0-1.0)
    pub confidence: f32,
}

/// 识别统计信息
#[derive(Debug, Clone)]
pub struct IdentificationStatistics {
    /// 总行数
    pub total_rows: usize,
    /// 各类型行数统计
    pub row_type_counts: HashMap<RowType, usize>,
    /// 未知类型行数
    pub unknown_count: usize,
    /// 识别成功率
    pub success_rate: f32,
}

impl IdentificationStatistics {
    /// 从识别结果列表创建统计信息
    pub fn from_results(results: &[RowIdentificationResult]) -> Self {
        let total_rows = results.len();
        let mut row_type_counts = HashMap::new();
        let mut unknown_count = 0;
        
        for result in results {
            *row_type_counts.entry(result.row_type.clone()).or_insert(0) += 1;
            if result.row_type == RowType::Unknown {
                unknown_count += 1;
            }
        }
        
        let success_rate = if total_rows > 0 {
            (total_rows - unknown_count) as f32 / total_rows as f32
        } else {
            0.0
        };
        
        Self {
            total_rows,
            row_type_counts,
            unknown_count,
            success_rate,
        }
    }
}
