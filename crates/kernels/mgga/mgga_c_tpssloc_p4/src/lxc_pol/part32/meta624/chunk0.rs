//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2032/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2032<F: Float>(t86590: F, t25: F, t40772: F, t1408: F, t2752: F, t2: F, t193: F, t201: F, t7540: F, t870: F, t25353: F, t25213: F, t6547: F) -> (F, F, F, F, F, F, F, F) {
    let t86591 = F::new(2.0) / F::new(3.0) * t86590;
    let t86716 = t40772 * t25;
    let t86721 = t2752 * t1408;
    let t86730 = t2752 * t2;
    let t86736 = t193 * t201 * t7540;
    let t86753 = t870 * t2;
    let t86836 = t25353 * t2752;
    let t86843 = t6547 * t25213;
    (t86591, t86716, t86721, t86730, t86736, t86753, t86836, t86843)
}
