//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1206/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1206<F: Float>(t40611: F, t8492: F, t26161: F, t26163: F, t31086: F, t7685: F, t31246: F, t7688: F, t1388: F, t7752: F, t26162: F, t33082: F, t6876: F, t1983: F, t55242: F, t8493: F) -> (F, F, F, F, F, F) {
    let t120684 = t8492 * t40611;
    let t120687 = 6.0 * t26161 * t120684 * t26163;
    let t120691 = 3.0 * t7685 * t31086;
    let t120692 = t31246 * t7688;
    let t120694 = t7752 * t1388;
    let t120697 = 4.0 * t26161 * t26162 * t120694;
    let t120699 = 2.0 * t6876 * t33082;
    let t120702 = 2.0 * t1983 * t8493 * t55242;
    (t120687, t120691, t120692, t120697, t120699, t120702)
}
