//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1374/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1374<F: Float>(t11159: F, t11502: F, t11662: F, t11721: F, t11774: F, t11805: F, t1218: F, t1232: F, t2121: F, t2134: F, t24655: F, t24685: F, t24716: F, t24729: F, t24736: F, t3496: F, t3508: F, t3511: F, t3527: F, t3587: F, t460: F, t4899: F, t7320: F, t7331: F, t7345: F, t86120: F, t86122: F, t86124: F, t86126: F, t86129: F, t86136: F, t86140: F, t86155: F, t86158: F, t86182: F, t86184: F, t86191: F, t86194: F, t86199: F, t86204: F, t86208: F, t86214: F, t86253: F, t86262: F, t86266: F, t86269: F, t86273: F, t86275: F, t86278: F, t86282: F, t86317: F, t86347: F, t86373: F) -> F {
    let t86376 = -F::cast_from(0.30279567070605293142e-3_f64) * t24685 * t24655 + F::new(5.0) / F::new(2304.0) * t7345 * t11774 - F::cast_from(0.30279567070605293142e-3_f64) * t86204 * t7331 + t24716 * t3496 / F::new(512.0) + t24729 * t11662 / F::new(256.0) + F::cast_from(0.60559134141210586284e-3_f64) * t86194 * t7331 - F::cast_from(0.30279567070605293142e-3_f64) * t86199 * t7331 + t86126 * t1218 / F::new(512.0) - t86129 * t1232 / F::new(768.0) - t24736 * t3527 / F::new(768.0) + F::new(5.0) / F::new(2304.0) * t24736 * t3587 - t7345 * t11805 / F::new(2304.0) + t86140 * t3511 / F::new(256.0) + t86373 + t86347 + t86317 - F::cast_from(0.30279567070605293142e-3_f64) * t86282 + t86278 + t86273 / F::new(768.0) - t86275 / F::new(2304.0) - F::cast_from(0.30279567070605293142e-3_f64) * t86269 + F::cast_from(0.60559134141210586284e-3_f64) * t86266 - F::cast_from(0.60559134141210586284e-3_f64) * t86262 + t86253 + t86191 + t86184 / F::new(432.0) + t86182 - t86136 / F::new(576.0) + t86122 / F::new(384.0) - t86124 / F::new(576.0) + F::new(5.0) / F::new(3456.0) * t86120 + t2121 * t4899 * t11159 / F::new(72.0) - F::cast_from(0.10093189023535097714e-3_f64) * t2134 * t11502 * t460 * t7320 + F::cast_from(0.60559134141210586284e-3_f64) * t86155 * t86208 * t86158 * t11721 - F::cast_from(0.60559134141210586284e-3_f64) * t86155 * t86214 * t86158 * t3508;
    t86376
}
