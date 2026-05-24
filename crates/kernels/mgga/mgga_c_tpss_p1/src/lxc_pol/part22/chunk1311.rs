//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1311/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1311<F: Float>(t236: F, t339: F, t61038: F, t10782: F, t10786: F, t19703: F, t10809: F, t17964: F, t3671: F, t61033: F, t10602: F, t10575: F) -> (F, F, F, F, F, F) {
    let t63920 = t339 * t61038 * t236;
    let t63921 = t63920 * t10782;
    let t63923 = t19703 * t10786;
    let t63925 = t17964 * t10809;
    let t63928 = t61033 * t3671;
    let t63930 = t17964 * t10602;
    let t63932 = t17964 * t10575;
    (t63921, t63923, t63925, t63928, t63930, t63932)
}
