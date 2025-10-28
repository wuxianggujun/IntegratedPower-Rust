// Recognition Rules Module
pub mod color_rule;
pub mod text_pattern_rule;
pub mod merge_state_rule;
pub mod composite_rule;

pub use color_rule::ColorRule;
pub use text_pattern_rule::{TextPattern, TextPatternRule};
pub use merge_state_rule::{MergeRequirement, MergeStateRule};
pub use composite_rule::{CompositeLogic, CompositeRule};
