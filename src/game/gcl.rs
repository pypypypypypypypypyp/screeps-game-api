//! Global Control Level information.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game.gcl)
use crate::constants::{GCL_MULTIPLY, GCL_POW};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "gcl")]
    type Gcl;

    /// Your current Global Control Level, which determines the number of rooms
    /// you are allowed to claim.
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "gcl", static_method_of = Gcl, getter, js_name = level)]
    fn level() -> u32;

    /// Your progress toward the next Global Control Level.
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "gcl", static_method_of = Gcl, getter, js_name = progress)]
    fn progress() -> f64;

    /// Total progress needed to reach the next Global Control Level.
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "gcl", static_method_of = Gcl, getter, js_name = progressTotal)]
    fn progress_total() -> f64;
}

pub fn level() -> u32 {
    Gcl::level()
}

pub fn progress() -> f64 {
    Gcl::progress()
}

pub fn progress_total() -> f64 {
    Gcl::progress_total()
}

/// Provides the total number of control points needed to achieve each level of
/// GCL.
///
/// Calculates the total number of control points needed to achieve a given
/// Global Control Level. The resulting value for your current level, added to
/// your [`GclInfo::progress`], would calculate your total lifetime control
/// points.
pub fn total_for_level(level: u32) -> f64 {
    // formula from
    // https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L117
    ((level - 1) as f64).powf(GCL_POW as f64) * GCL_MULTIPLY as f64
}
