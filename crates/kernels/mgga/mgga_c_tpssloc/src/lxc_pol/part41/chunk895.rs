//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 895/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk895<F: Float>(t1500: F, t2693: F, t4163: F, t838: F, t120: F, t4233: F, t2642: F, t4166: F, t2628: F, t836: F, t812: F, t4184: F, t242: F, t9972: F, t2639: F, t4236: F) -> (F, F, F, F, F, F, F) {
    let t13234 = t1500 * t2693;
    let t13237 = 7.0 / 2304.0 * t4163 * t838;
    let t13242 = t120 * t4233;
    let t13251 = t4166 * t2642;
    let t13257 = t2628 * t836;
    let t13258 = t812 * t13257;
    let t13260 = 7.0 / 1152.0 * t13258 * t4184;
    let t13261 = t9972 * t242;
    let t13262 = t812 * t13261;
    let t13275 = 7.0 / 2304.0 * t2639 * t4236;
    (t13234, t13237, t13242, t13251, t13260, t13262, t13275)
}
