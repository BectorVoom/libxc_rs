//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 746/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk746<F: Float>(t19095: F, t3515: F, t1243: F, t19045: F, t225: F, t6151: F, t6153: F, t6239: F, t3640: F, t6270: F, t5385: F, t604: F, t1409: F, t65: F, t67: F, t111: F, t5449: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19096 = t3515 * t19095;
    let t19201 = t19045 * t1243;
    let t19232 = t6151 * t225;
    let t19234 = t6153 * t225;
    let t19249 = t6239 * t225;
    let t19267 = t6270 * t3640;
    let t19299 = t5385 * t604;
    let t19322 = t1409 * t65 * t67;
    let t19451 = t5449 * t111;
    (t19096, t19201, t19232, t19234, t19249, t19267, t19299, t19322, t19451)
}
