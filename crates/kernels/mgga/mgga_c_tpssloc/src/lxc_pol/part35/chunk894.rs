//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 894/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk894<F: Float>(t1343: F, t20554: F, t820: F, t1799: F, t6347: F, t3870: F, t20489: F, t550: F, t20416: F, t210: F, t214: F, t20356: F, t221: F, t5196: F, t12188: F, t12194: F, t12196: F, t12215: F, t12236: F, t1315: F, t16078: F, t16108: F, t16119: F, t19768: F, t19776: F, t19779: F, t19791: F, t5195: F) -> (F, F, F, F, F, F) {
    let t20556 = t1343 * t820 * t20554;
    let t20563 = t1799 * t6347;
    let t20565 = t3870 * t820 * t20563;
    let t20568 = t20489 * t550;
    let t20570 = t1343 * t820 * t20568;
    let t20576 = t210 * t214 * t20416;
    let t20582 = t210 * t214 * t20356;
    let t20586 = t221 * t5196 * t6347;
    let t20594 = -0.16666666666666666666e-2 * t1315 * t20576 - t12188 - 0.74999999999999999997e-2 * t19768 + 0.24999999999999999999e-2 * t19776 - t12194 + t12196 - 0.19999999999999999999e-1 * t12215 * t20582 + 0.14999999999999999999e-1 * t5195 * t20586 - 0.34999999999999999998e-1 * t19779 + 0.11666666666666666666e-1 * t19791 - 0.38888888888888888888e-1 * t16078 - t12236 - 0.15833333333333333333e-1 * t16108 + 0.49999999999999999998e-2 * t16119;
    (t20556, t20563, t20565, t20568, t20570, t20594)
}
