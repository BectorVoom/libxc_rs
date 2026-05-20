//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2273/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2273<F: Float>(t23384: F, t28681: F, t1054: F, t5943: F, t1921: F, t5914: F, t6688: F, t225: F, t28505: F, t28496: F, t1066: F, t17582: F, t18165: F, t23346: F, t25406: F, t25732: F, t25757: F, t25758: F, t25826: F, t28697: F, t28713: F, t3026: F, t4557: F, t6687: F, t6691: F, t6704: F, t6705: F, t82436: F, t986: F) -> (F, F) {
    let t99205 = t23384 * t28681;
    let t99209 = t1054 * t5943;
    let t99210 = t1921 * t99209;
    let t99214 = t6688 * t5914;
    let t99221 = t28505 * t225;
    let t99230 = t23384 * t28496;
    let t99238 = -F::cast_from(0.54831135561607547883e-2_f64) * t99205 + t82436 + F::new(2.0) * t3026 * t28713 + F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t986 * t99210 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t99214 * t6691 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t25406 * t25826 - t99221 * t1066 - F::new(12.0) * t25757 * t25758 * t17582 - F::new(2.0) * t4557 * t25732 - F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t28496 + F::cast_from(0.54831135561607547883e-2_f64) * t99230 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t6704 * t6705 * t18165 - F::new(6.0) * t3026 * t28697;
    (t99209, t99238)
}
