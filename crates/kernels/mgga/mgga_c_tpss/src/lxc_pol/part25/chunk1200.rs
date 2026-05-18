//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1200/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1200<F: Float>(t136: F, t238: F, t1693: F, t215: F, t3683: F, t3622: F, t5547: F, t17954: F, t236: F, t339: F) -> (F, F, F, F, F, F) {
    let t19695 = t238 * t136;
    let t19696 = t1693 * t19695;
    let t19697 = t215 * t3683;
    let t19698 = t19696 * t19697;
    let t19700 = t5547 * t3622;
    let t19703 = t339 * t17954 * t236;
    (t19695, t19696, t19697, t19698, t19700, t19703)
}
