//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1205/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1205<F: Float>(t8449: F, t8944: F, t26164: F, t3701: F, t5187: F, t1983: F, t31084: F, t26504: F, t8450: F, t33157: F, t6876: F, t120145: F, t1874: F, t120148: F, t31036: F, t7685: F) -> (F, F, F, F, F, F, F) {
    let t120664 = t8449 * t8944;
    let t120665 = t120664 * t26164;
    let t120669 = t3701 * t5187;
    let t120672 = 3.0 * t1983 * t31084 * t120669;
    let t120675 = t8450 * t26504;
    let t120677 = t6876 * t33157;
    let t120678 = t120145 * t1874;
    let t120680 = t120148 * t1874;
    let t120683 = 2.0 * t7685 * t31036;
    (t120665, t120672, t120675, t120677, t120678, t120680, t120683)
}
