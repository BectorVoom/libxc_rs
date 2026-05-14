//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1199/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1199<F: Float>(t1388: F, t7752: F, t1307: F, t26179: F, t8327: F, t31058: F, t7458: F, t19456: F, t4028: F, t12725: F, t20173: F, t33193: F, t3941: F, t4072: F, t8326: F, t16524: F, t31285: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t120694 = t7752 * t1388;
    let t120705 = t7752 * t1307;
    let t120719 = 2.0 * t26179 * t8327;
    let t120721 = 2.0 * t7458 * t31058;
    let t120728 = 2.0 * t19456 * t8327;
    let t120730 = 2.0 * t4028 * t31058;
    let t120735 = 2.0 * t12725 * t8327;
    let t120800 = 27.0 * t20173 * t33193;
    let t120803 = 27.0 * t3941 * t8326 * t4072;
    let t120807 = 27.0 * t16524 * t31285;
    (t120694, t120705, t120719, t120721, t120728, t120730, t120735, t120800, t120803, t120807)
}
