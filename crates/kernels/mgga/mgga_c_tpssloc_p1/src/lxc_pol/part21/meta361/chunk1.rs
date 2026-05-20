//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1782/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1782<F: Float>(t13263: F, t4282: F, t2633: F, t9632: F, t2732: F, t4234: F, t2679: F, t4295: F, t1519: F, t2627: F, t10076: F, t1510: F) -> (F, F, F, F, F, F, F) {
    let t13398 = t4282 * t13263;
    let t13401 = t4282 * t2633;
    let t13404 = t4282 * t9632;
    let t13407 = t2732 * t4234;
    let t13414 = t4295 * t2679;
    let t13416 = t2627 * t1519;
    let t13417 = t13416 * t2633;
    let t13423 = t10076 * t1510;
    (t13398, t13401, t13404, t13407, t13414, t13417, t13423)
}
