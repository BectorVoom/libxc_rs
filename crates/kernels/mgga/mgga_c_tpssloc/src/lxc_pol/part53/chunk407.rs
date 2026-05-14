//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 407/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk407<F: Float>(t363: F, t368: F, t1017: F, t67: F, t1058: F, t1044: F, t820: F, t374: F, t376: F, t677: F, t370: F, t1032: F, t1036: F, t121: F, t1023: F, t248: F) -> (F, F, F, F, F, F, F) {
    let t3067 = t363 * t368;
    let t3068 = t1017 * t67;
    let t3069 = t3067 * t3068;
    let t3070 = t1058 * t3069;
    let t3071 = t820 * t1044;
    let t3082 = t374 * t677 * t376;
    let t3084 = t370 * t3082 / 13824.0;
    let t3092 = t1032 * t1036;
    let t3101 = t121 * t376;
    let t3103 = t248 * t3101 * t1023;
    (t3068, t3070, t3071, t3084, t3092, t3101, t3103)
}
