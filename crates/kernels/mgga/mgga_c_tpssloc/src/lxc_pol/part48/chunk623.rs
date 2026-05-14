//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 623/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk623<F: Float>(t12734: F, t1874: F, t2314: F, t6525: F, t2006: F, t3752: F, t1323: F, t6955: F, t2015: F, t3888: F, t12021: F, t1887: F, t6916: F) -> (F, F, F, F, F, F, F) {
    let t22616 = 4.0 * t12734 * t1874;
    let t22618 = 4.0 * t2314 * t6525;
    let t22622 = t3752 * t2006;
    let t22624 = t1323 * t6955;
    let t22629 = t2015 * t3888;
    let t22630 = t12021 * t22629;
    let t22633 = t6916 * t1887;
    (t22616, t22618, t22622, t22624, t22629, t22630, t22633)
}
