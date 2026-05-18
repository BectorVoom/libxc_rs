//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1019/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1019<F: Float>(t2697: F, t4257: F, t4119: F, t776: F, t2701: F, t820: F, t1484: F, t2553: F, t2563: F, t4159: F, t119: F, t12971: F) -> (F, F, F, F, F) {
    let t13190 = F::new(35.0) / F::new(576.0) * t2697 * t4257;
    let t13191 = t4119 * t776;
    let t13193 = t2701 * t820 * t13191;
    let t13196 = t1484 * t2553;
    let t13198 = t2701 * t820 * t13196;
    let t13202 = F::new(7.0) / F::new(72.0) * t2563 * t4159;
    let t13203 = t119 * t12971;
    (t13190, t13193, t13198, t13202, t13203)
}
