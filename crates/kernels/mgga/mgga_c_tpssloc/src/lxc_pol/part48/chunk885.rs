//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 885/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk885<F: Float>(t22597: F, t8607: F, t12734: F, t8533: F, t2314: F, t31772: F, t1874: F, t91857: F, t26977: F, t6525: F, t22585: F, t31304: F, t7000: F, t6997: F, t649: F, t6534: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115700 = 6.0 * t8607 * t22597;
    let t115702 = 4.0 * t12734 * t8533;
    let t115704 = 4.0 * t2314 * t31772;
    let t115708 = 2.0 * t91857 * t1874;
    let t115712 = 4.0 * t26977 * t6525;
    let t115716 = 3.0 * t8607 * t22585;
    let t115718 = 2.0 * t31304 * t7000;
    let t115721 = 2.0 * t31304 * t6997;
    let t115723 = t649 * t6534;
    (t115700, t115702, t115704, t115708, t115712, t115716, t115718, t115721, t115723)
}
