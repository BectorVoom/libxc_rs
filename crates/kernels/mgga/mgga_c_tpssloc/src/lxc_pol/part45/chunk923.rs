//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 923/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk923<F: Float>(t31518: F, t650: F, t2312: F, t8595: F, t22592: F, t8607: F, t31759: F, t6876: F, t22573: F, t8606: F, t22575: F, t31526: F, t114360: F, t115766: F, t115771: F, t115773: F, t115777: F, t115781: F, t115809: F, t115915: F, t2040: F, t2075: F, t22559: F, t24433: F, t24442: F, t26103: F, t574: F, t6517: F, t6862: F, t7040: F, t7050: F, t90044: F) -> (F,) {
    let t115919 = 2.0 * t650 * t31518;
    let t115920 = t2312 * t8595;
    let t115922 = 6.0 * t8607 * t22592;
    let t115924 = 6.0 * t6876 * t31759;
    let t115925 = t8606 * t22573;
    let t115927 = 6.0 * t115925 * t22575;
    let t115929 = 2.0 * t6876 * t31526;
    let t115934 = -2.0 * t90044 * t2040 - 4.0 * t26103 * t7050 + t115766 - 2.0 * t7040 * t6862 - t115771 - t22559 * t2075 - t115773 + t115777 - t115781 + (t115809 + t115915) * t574 - t115919 - t115920 + t115922 + t115924 - t115927 - t115929 - 6.0 * t114360 * t24433 - 2.0 * t6517 * t24442;
    (t115934,)
}
