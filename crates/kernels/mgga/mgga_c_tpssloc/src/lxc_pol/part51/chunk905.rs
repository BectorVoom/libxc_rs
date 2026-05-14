//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 905/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk905<F: Float>(t1874: F, t19456: F, t4028: F, t6525: F, t5161: F, t6996: F, t1983: F, t1914: F, t193: F, t200: F, t25: F, t870: F, t4255: F, t16596: F, t22960: F, t1484: F, t606: F) -> (F, F, F, F, F, F, F, F) {
    let t25005 = 2.0 * t19456 * t1874;
    let t25007 = 2.0 * t4028 * t6525;
    let t25010 = t6996 * t5161;
    let t25011 = t1983 * t25010;
    let t25013 = t193 * t200 * t1914;
    let t25014 = t870 * t25;
    let t25015 = t25014 * t4255;
    let t25021 = t22960 * t16596;
    let t25024 = t606 * t1484;
    (t25005, t25007, t25010, t25011, t25013, t25015, t25021, t25024)
}
