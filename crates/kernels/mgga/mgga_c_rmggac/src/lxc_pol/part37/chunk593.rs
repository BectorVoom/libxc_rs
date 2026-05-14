//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 593/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk593<F: Float>(t40927: F, t838: F, t1614: F, t664: F, t1587: F, t2067: F, t26: F, t2367: F, t333: F, t1652: F, t2123: F, t551: F, t570: F, t321: F, t118: F, t25809: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t40932 = t838 * t40927;
    let t40940 = t664 * t1614;
    let t40983 = t664 * t1587;
    let t40998 = t2067 * t26;
    let t41006 = t2367 * t333;
    let t41015 = t664 * t1652;
    let t41059 = t2123 * t551;
    let t41063 = t2123 * t570;
    let t41091 = t2367 * t321;
    let t41116 = t118 * t25809;
    (t40932, t40940, t40983, t40998, t41006, t41015, t41059, t41063, t41091, t41116)
}
