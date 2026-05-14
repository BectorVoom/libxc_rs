//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 504/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk504<F: Float>(t3061: F, t61: F, t248: F, t2771: F, t363: F, t368: F, t1017: F, t67: F, t1058: F, t1044: F, t820: F) -> (F, F, F, F, F, F, F) {
    let t3062 = t61 * t3061;
    let t3064 = t248 * t3062 * t2771;
    let t3067 = t363 * t368;
    let t3068 = t1017 * t67;
    let t3069 = t3067 * t3068;
    let t3070 = t1058 * t3069;
    let t3071 = t820 * t1044;
    (t3062, t3064, t3067, t3068, t3069, t3070, t3071)
}
