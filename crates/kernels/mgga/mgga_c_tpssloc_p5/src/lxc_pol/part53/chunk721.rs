//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 721/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk721<F: Float>(t2039: F, t88: F, t1390: F, t2094: F, t2229: F, t3: F, t2239: F, t601: F, t83: F, t84: F, t85: F, t24: F) -> (F, F, F, F, F, F, F) {
    let t9012 = t88 * t2039;
    let t9016 = t2094 * t1390;
    let t9222 = t2229 * t3;
    let t9223 = F::new(1.0) / t9222;
    let t9231 = t601 * t2239;
    let t9238 = F::new(1.0) / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    (t9012, t9016, t9222, t9223, t9231, t9238, t9239)
}
