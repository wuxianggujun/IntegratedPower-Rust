// Data Engine 模块
pub mod data_engine;
pub mod recognition_rule;
pub mod rules;
pub mod file_type_profile;
pub mod row_identifier;
pub mod excel_extractor;
pub mod identification_error;

pub use recognition_rule::RecognitionRule;
pub use rules::{
    ColorRule, TextPattern, TextPatternRule, 
    MergeRequirement, MergeStateRule,
    CompositeLogic, CompositeRule,
};
pub use file_type_profile::{FileTypeProfile, RowTypeDefinition};
pub use row_identifier::RowTypeIdentifier;
pub use excel_extractor::ExcelExtractor;
pub use identification_error::{IdentificationError, IdentificationResult};
