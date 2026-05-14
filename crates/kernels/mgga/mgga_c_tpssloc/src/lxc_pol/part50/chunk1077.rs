//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1077/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1077<F: Float>(t23665: F, t32935: F, t113508: F, t113511: F, t113526: F, t1920: F, t1948: F, t23327: F, t23346: F, t23613: F, t23670: F, t25499: F, t25523: F, t25567: F, t25705: F, t30885: F, t3186: F, t32927: F, t32928: F, t32943: F, t345: F, t4673: F, t6797: F, t6799: F, t6800: F) -> (F,) {
    let t119221 = t23665 * t32935;
    let t119232 = -0.16449340668482264365e-1 * t6797 * t25523 * t30885 + 0.16449340668482264365e-1 * t1920 * t345 * t1948 * t25705 - 0.54831135561607547883e-2 * t113508 + 2.0 * t3186 * t32943 * t4673 + 0.54831135561607547883e-2 * t113511 + 0.16449340668482264365e-1 * t6797 * t6799 * t25567 * t6800 - 0.54831135561607547883e-2 * t23327 * t23613 * t32927 + 0.54831135561607547883e-2 * t119221 + 0.16449340668482264365e-1 * t6797 * t6799 * t25499 * t6800 - 0.43864908449286038307e-1 * t23670 * t32935 + 0.54831135561607547883e-2 * t113526 - 0.14621636149762012769e-1 * t23346 * t32928;
    (t119232,)
}
