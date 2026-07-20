#![allow(dead_code)]

//! Shared visual tokens for the native desktop renderer.
//!
//! The surface is configured with an sRGB format, so these are kept as sRGB
//! values and passed to wgpu unchanged.

use std::time::Duration;

pub(crate) type Color = [f32; 4];

pub(crate) const fn rgb(hex: u32) -> Color {
    let r = ((hex >> 16) & 0xff) as f32 / 255.0;
    let g = ((hex >> 8) & 0xff) as f32 / 255.0;
    let b = (hex & 0xff) as f32 / 255.0;
    // The primitive shader writes directly into an sRGB target, which expects
    // linear vertex colors. Squaring is a close const-friendly approximation
    // of the sRGB transfer curve for the UI palette.
    [r * r, g * g, b * b, 1.0]
}

pub(crate) const fn rgba(hex: u32, alpha: f32) -> Color {
    let mut color = rgb(hex);
    color[3] = alpha;
    color
}

// Backgrounds and text.
pub(crate) const BG_BASE: Color = rgb(0x131519);
pub(crate) const BG_RAISED: Color = rgb(0x1A1D22);
pub(crate) const BG_OVERLAY: Color = rgb(0x21252C);
pub(crate) const BG_INSET: Color = rgb(0x0D0F12);
pub(crate) const TEXT_PRIMARY: Color = rgb(0xD9DCE2);
pub(crate) const TEXT_SECONDARY: Color = rgb(0x9AA1AC);
pub(crate) const TEXT_MUTED: Color = rgb(0x7B8390);
pub(crate) const TEXT_DISABLED: Color = rgb(0x565D66);
pub(crate) const TEXT_INVERSE: Color = rgb(0x14161A);
pub(crate) const TEXT_ON_ERROR: Color = rgb(0xFFFFFF);

// Borders and accent.
pub(crate) const BORDER_DEFAULT: Color = rgb(0x262B33);
pub(crate) const BORDER_STRONG: Color = rgb(0x343B45);
pub(crate) const BORDER_FOCUS: Color = rgb(0xE3A832);
pub(crate) const ACCENT: Color = rgb(0xE3A832);
pub(crate) const ACCENT_HOVER: Color = rgb(0xF0BA4C);
pub(crate) const ACCENT_PRESSED: Color = rgb(0xC99026);
pub(crate) const ACCENT_SURFACE: Color = rgb(0x2B2417);
pub(crate) const ACCENT_BORDER: Color = rgb(0x4D4020);

// Semantic colors.
pub(crate) const SUCCESS: Color = rgb(0x5FBF7F);
pub(crate) const SUCCESS_SURFACE: Color = rgb(0x182A1F);
pub(crate) const SUCCESS_BORDER: Color = rgb(0x2C4A37);
pub(crate) const WARNING: Color = rgb(0xE5CE4E);
pub(crate) const WARNING_SURFACE: Color = rgb(0x2C2818);
pub(crate) const WARNING_BORDER: Color = rgb(0x4A4322);
pub(crate) const ERROR: Color = rgb(0xE0685F);
pub(crate) const ERROR_SURFACE: Color = rgb(0x2E1B19);
pub(crate) const ERROR_BORDER: Color = rgb(0x52302B);
pub(crate) const INFO: Color = rgb(0x6FA8DC);
pub(crate) const INFO_SURFACE: Color = rgb(0x182230);
pub(crate) const INFO_BORDER: Color = rgb(0x2A3D54);

// Component and syntax colors.
pub(crate) const SELECTION: Color = rgba(0xE3A832, 0.30);
pub(crate) const PANE_BORDER_ACTIVE: Color = rgba(0xE3A832, 0.45);
pub(crate) const SCROLLBAR_THUMB: Color = rgb(0x343B45);
pub(crate) const SCROLLBAR_THUMB_HOVER: Color = rgb(0x454D59);
pub(crate) const SHADOW_COLOR: Color = rgba(0x000000, 0.18);
pub(crate) const SYNTAX_KEYWORD: Color = rgb(0xC08FE0);
pub(crate) const SYNTAX_FUNCTION: Color = INFO;
pub(crate) const SYNTAX_TYPE: Color = rgb(0x56B6C2);
pub(crate) const SYNTAX_STRING: Color = rgb(0x98C379);
pub(crate) const SYNTAX_NUMBER: Color = rgb(0xD19A66);
pub(crate) const SYNTAX_CONSTANT: Color = rgb(0xE0708A);
pub(crate) const SYNTAX_COMMENT: Color = rgb(0x5C6673);
pub(crate) const SYNTAX_OPERATOR: Color = rgb(0x9AA4B2);

// Type scale.
pub(crate) const TYPE_DISPLAY_SIZE: f32 = 20.0;
pub(crate) const TYPE_BODY_SIZE: f32 = 14.0;
pub(crate) const TYPE_BODY_LINE: f32 = 22.0;
pub(crate) const TYPE_LABEL_SIZE: f32 = 12.0;
pub(crate) const TYPE_CAPTION_SIZE: f32 = 11.0;
pub(crate) const TYPE_CODE_SIZE: f32 = 13.0;
pub(crate) const TYPE_CODE_SMALL_SIZE: f32 = 12.0;
pub(crate) const TYPE_MONO_SMALL_SIZE: f32 = 11.0;
pub(crate) const TYPE_TITLE_SIZE: f32 = 13.0;

// Geometry.
pub(crate) const BORDER_W: f32 = 1.0;
pub(crate) const RAIL_W: f32 = 3.0;
pub(crate) const RADIUS_SM: f32 = 4.0;
pub(crate) const RADIUS_MD: f32 = 6.0;
pub(crate) const RADIUS_LG: f32 = 8.0;
pub(crate) const COLUMN_MAX: f32 = 760.0;
pub(crate) const MARGIN_MIN: f32 = 24.0;
pub(crate) const TITLEBAR_H: f32 = 40.0;
pub(crate) const STATUSBAR_H: f32 = 24.0;
pub(crate) const TOOLCARD_H: f32 = 36.0;
pub(crate) const COMPOSER_MIN_H: f32 = 56.0;
pub(crate) const COMPOSER_MAX_H: f32 = 240.0;
pub(crate) const COMPOSER_LINE_STEP: f32 = 22.0;
pub(crate) const TURN_GAP: f32 = 24.0;
pub(crate) const BLOCK_GAP: f32 = 12.0;
pub(crate) const TRANSCRIPT_TOP_INSET: f32 = 24.0;
pub(crate) const SCROLLBAR_W: f32 = 8.0;
pub(crate) const CARET_W: f32 = 2.0;
pub(crate) const STREAM_CARET_W: f32 = 9.0;
pub(crate) const STREAM_CARET_H: f32 = 18.0;

// Motion.
pub(crate) const DUR_HOVER: Duration = Duration::from_millis(80);
pub(crate) const DUR_FAST: Duration = Duration::from_millis(120);
pub(crate) const DUR_STATE: Duration = Duration::from_millis(160);
pub(crate) const DUR_EXPAND: Duration = Duration::from_millis(200);
pub(crate) const DUR_ENTER: Duration = Duration::from_millis(240);
pub(crate) const DUR_REVEAL: Duration = Duration::from_millis(320);
pub(crate) const DUR_PULSE: Duration = Duration::from_millis(900);
pub(crate) const DUR_SPIN: Duration = Duration::from_millis(1000);
pub(crate) const STAGGER_MS: u64 = 70;
pub(crate) const CARET_BLINK: Duration = Duration::from_millis(530);

pub(crate) const EASE_STANDARD: [f32; 4] = [0.25, 0.10, 0.25, 1.00];
pub(crate) const EASE_OUT: [f32; 4] = [0.16, 1.00, 0.30, 1.00];
pub(crate) const EASE_IN_OUT: [f32; 4] = [0.45, 0.00, 0.15, 1.00];
