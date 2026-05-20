//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1244/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1244<F: Float>(t1453: F, t81439: F, t111: F, t7758: F, t1408: F, t2752: F, t193: F, t201: F, t7540: F, t25345: F, t82038: F, t1519: F, t213: F, t225: F) -> (F, F, F, F, F, F) {
    let t86586 = t81439 * t1453;
    let t86647 = t7758 * t111;
    let t86721 = t2752 * t1408;
    let t86736 = t193 * t201 * t7540;
    let t86870 = t82038 * t25345;
    let t86873 = t213 * t1519 * t225;
    (t86586, t86647, t86721, t86736, t86870, t86873)
}
