//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 452/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk452<F: Float>(t1137: F, t6052: F, t3359: F, t6036: F, t3363: F, t4721: F, t5973: F, t5977: F, t5981: F, t449: F, t1694: F, t1156: F, t3383: F, t3390: F, t4770: F, t5993: F, t6000: F, t6006: F, t6008: F, t6012: F, t6015: F, t6018: F) -> (F, F, F, F, F, F) {
    let t6053 = t6052 * t1137;
    let t6056 = t6036 * t3359;
    let t6063 = t3363 - 0.61805555555555555556e-2 * t4721 - 0.61805555555555555555e-2 * t5973 + 0.18541666666666666667e-1 * t5977 + 0.92708333333333333333e-2 * t5981;
    let t6064 = t6063 * t449;
    let t6068 = t1694 * t1694;
    let t6069 = t6068 * t1156;
    let t6084 = -0.1294625e1 * t5993 + 0.258925e1 * t6000 + t3383 - 0.20128333333333333334e0 * t4721 - 0.20128333333333333333e0 * t5973 + 0.60385e0 * t5977 + 0.301925e0 * t5981 + 0.82524375e-1 * t6006 + 0.16504875e0 * t6008 + t3390 - 0.11038e0 * t4770 - 0.27595e-1 * t6012 + 0.16557e0 * t6015 + 0.82785e-1 * t6018;
    (t6053, t6056, t6064, t6068, t6069, t6084)
}
