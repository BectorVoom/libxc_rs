//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2165/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2165<F: Float>(t22756: F, t6422: F, t22783: F, t6431: F, t1831: F, t91160: F, t19815: F, t6951: F, t1369: F, t91136: F, t91138: F, t91141: F, t97236: F, t97238: F, t97240: F, t97242: F, t97244: F, t97247: F, t97249: F, t97251: F, t97253: F, t97255: F, t97257: F) -> F {
    let t97259 = t22756 * t6422;
    let t97261 = t22783 * t6431;
    let t97263 = t91160 * t1831;
    let t97265 = t19815 * t6951;
    let t97266 = t97265 * t1369;
    let t97268 = F::cast_from(0.80745512188280781708e-3_f64) * t97236 - F::cast_from(0.16956557559538964158e-1_f64) * t97238 + F::new(7.0) / F::new(1152.0) * t97240 - t97242 / F::new(1536.0) - t97244 / F::new(1536.0) - t97247 / F::new(1536.0) - t97249 / F::new(768.0) - t97251 / F::new(768.0) + F::new(7.0) / F::new(2304.0) * t97253 - F::new(5.0) / F::new(64.0) * t97255 + F::new(5.0) / F::new(192.0) * t97257 - t97259 / F::new(1536.0) + F::new(7.0) / F::new(576.0) * t97261 + F::new(7.0) / F::new(288.0) * t97263 - t97266 / F::new(384.0) + t91136 + t91138 - t91141;
    t97268
}
