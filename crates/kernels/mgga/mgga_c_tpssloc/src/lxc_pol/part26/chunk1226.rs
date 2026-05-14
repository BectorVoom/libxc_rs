//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1226/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1226<F: Float>(t11159: F, t11502: F, t11662: F, t11721: F, t11774: F, t11805: F, t1218: F, t1232: F, t2121: F, t2134: F, t24655: F, t24685: F, t24716: F, t24729: F, t24736: F, t3496: F, t3508: F, t3511: F, t3527: F, t3587: F, t460: F, t4899: F, t7320: F, t7331: F, t7345: F, t86120: F, t86122: F, t86124: F, t86126: F, t86129: F, t86136: F, t86140: F, t86155: F, t86158: F, t86182: F, t86184: F, t86191: F, t86194: F, t86199: F, t86204: F, t86208: F, t86214: F, t86253: F, t86262: F, t86266: F, t86269: F, t86273: F, t86275: F, t86278: F, t86282: F, t86317: F, t86347: F, t86373: F) -> (F,) {
    let t86376 = -0.30279567070605293142e-3 * t24685 * t24655 + 5.0 / 2304.0 * t7345 * t11774 - 0.30279567070605293142e-3 * t86204 * t7331 + t24716 * t3496 / 512.0 + t24729 * t11662 / 256.0 + 0.60559134141210586284e-3 * t86194 * t7331 - 0.30279567070605293142e-3 * t86199 * t7331 + t86126 * t1218 / 512.0 - t86129 * t1232 / 768.0 - t24736 * t3527 / 768.0 + 5.0 / 2304.0 * t24736 * t3587 - t7345 * t11805 / 2304.0 + t86140 * t3511 / 256.0 + t86373 + t86347 + t86317 - 0.30279567070605293142e-3 * t86282 + t86278 + t86273 / 768.0 - t86275 / 2304.0 - 0.30279567070605293142e-3 * t86269 + 0.60559134141210586284e-3 * t86266 - 0.60559134141210586284e-3 * t86262 + t86253 + t86191 + t86184 / 432.0 + t86182 - t86136 / 576.0 + t86122 / 384.0 - t86124 / 576.0 + 5.0 / 3456.0 * t86120 + t2121 * t4899 * t11159 / 72.0 - 0.10093189023535097714e-3 * t2134 * t11502 * t460 * t7320 + 0.60559134141210586284e-3 * t86155 * t86208 * t86158 * t11721 - 0.60559134141210586284e-3 * t86155 * t86214 * t86158 * t3508;
    (t86376,)
}
