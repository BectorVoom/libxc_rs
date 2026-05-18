//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 958/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk958<F: Float>(t32253: F, t8301: F, t2240: F, t31024: F, t32248: F, t9231: F, t32244: F, t39063: F, t9239: F, t31006: F, t39054: F, t31000: F, t32255: F) -> (F, F, F, F, F, F, F, F) {
    let t116918 = t8301 * t32253;
    let t116919 = t2240 * t116918;
    let t116920 = t116919 * t31024;
    let t116929 = t9231 * t32248;
    let t116932 = t39063 * t32244;
    let t116935 = t9239 * t116918;
    let t116936 = t116935 * t31006;
    let t116942 = t39054 * t32244;
    let t116945 = t31000 * t32255;
    (t116919, t116920, t116929, t116932, t116935, t116936, t116942, t116945)
}
