//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 916/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk916<F: Float>(t191: F, t192: F, t24026: F, t2020: F, t15904: F, t22574: F, t36740: F, t22579: F, t8607: F, t31668: F, t532: F, t1983: F, t6879: F, t2018: F, t24432: F, t3719: F) -> (F, F, F, F, F) {
    let t115765 = t24026 * t191 * t192;
    let t115766 = t115765 * t2020;
    let t115771 = 6.0 * t22574 * t36740 * t15904;
    let t115773 = t8607 * t22579;
    let t115774 = t532 * t31668;
    let t115777 = 6.0 * t1983 * t115774 * t6879;
    let t115781 = 3.0 * t22574 * t24432 * t2018 * t3719;
    (t115766, t115771, t115773, t115777, t115781)
}
