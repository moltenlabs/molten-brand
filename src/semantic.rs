//! Semantic color tokens.
//!
//! These colors convey meaning and are consistent across all products.
//! Use these for status indicators, alerts, and feedback.

use crate::color::Color;

// =============================================================================
// SUCCESS
// =============================================================================

/// Success color - indicates positive outcomes.
pub const SUCCESS: Color = Color::rgb(16, 185, 129); // #10B981

/// Success light variant.
pub const SUCCESS_LIGHT: Color = Color::rgb(209, 250, 229); // #D1FAE5

/// Success dark variant.
pub const SUCCESS_DARK: Color = Color::rgb(5, 150, 105); // #059669

// =============================================================================
// WARNING
// =============================================================================

/// Warning color - indicates caution needed.
pub const WARNING: Color = Color::rgb(245, 158, 11); // #F59E0B

/// Warning light variant.
pub const WARNING_LIGHT: Color = Color::rgb(254, 243, 199); // #FEF3C7

/// Warning dark variant.
pub const WARNING_DARK: Color = Color::rgb(217, 119, 6); // #D97706

// =============================================================================
// ERROR
// =============================================================================

/// Error color - indicates problems or failures.
pub const ERROR: Color = Color::rgb(239, 68, 68); // #EF4444

/// Error light variant.
pub const ERROR_LIGHT: Color = Color::rgb(254, 226, 226); // #FEE2E2

/// Error dark variant.
pub const ERROR_DARK: Color = Color::rgb(220, 38, 38); // #DC2626

// =============================================================================
// INFO
// =============================================================================

/// Info color - indicates neutral information.
pub const INFO: Color = Color::rgb(59, 130, 246); // #3B82F6

/// Info light variant.
pub const INFO_LIGHT: Color = Color::rgb(219, 234, 254); // #DBEAFE

/// Info dark variant.
pub const INFO_DARK: Color = Color::rgb(37, 99, 235); // #2563EB

// =============================================================================
// AGENT STATUS COLORS (Goblin-specific)
// =============================================================================

/// Agent status colors for Lair/Goblin.
pub mod agent {
    use crate::color::Color;

    /// Spawning state - purple.
    pub const SPAWNING: Color = Color::rgb(124, 58, 237); // #7C3AED

    /// Running state - green.
    pub const RUNNING: Color = Color::rgb(16, 185, 129); // #10B981

    /// Thinking/processing state - amber.
    pub const THINKING: Color = Color::rgb(245, 158, 11); // #F59E0B

    /// Complete state - cyan.
    pub const COMPLETE: Color = Color::rgb(6, 182, 212); // #06B6D4

    /// Failed state - red.
    pub const FAILED: Color = Color::rgb(239, 68, 68); // #EF4444

    /// Idle state - gray.
    pub const IDLE: Color = Color::rgb(113, 113, 122); // #71717A

    /// Paused state - purple light.
    pub const PAUSED: Color = Color::rgb(167, 139, 250); // #A78BFA
}

/// Semantic color struct for use in themes.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SemanticColors {
    /// Success color.
    pub success: Color,
    /// Warning color.
    pub warning: Color,
    /// Error color.
    pub error: Color,
    /// Info color.
    pub info: Color,
}

impl Default for SemanticColors {
    fn default() -> Self {
        Self {
            success: SUCCESS,
            warning: WARNING,
            error: ERROR,
            info: INFO,
        }
    }
}

impl SemanticColors {
    /// Create a new semantic colors instance with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            success: SUCCESS,
            warning: WARNING,
            error: ERROR,
            info: INFO,
        }
    }

    /// Get color by semantic name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<Color> {
        match name.to_lowercase().as_str() {
            "success" => Some(self.success),
            "warning" => Some(self.warning),
            "error" => Some(self.error),
            "info" => Some(self.info),
            _ => None,
        }
    }
}
