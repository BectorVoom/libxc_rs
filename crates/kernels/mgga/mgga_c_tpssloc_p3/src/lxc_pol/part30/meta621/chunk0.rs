//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2020/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2020<F: Float>(t2: F, t2752: F, t193: F, t201: F, t7540: F, t870: F, t25353: F, t25213: F, t6547: F, t4119: F, t857: F, t23168: F, t25342: F) -> (F, F, F, F, F, F, F) {
    let t86730 = t2752 * t2;
    let t86736 = t193 * t201 * t7540;
    let t86753 = t870 * t2;
    let t86836 = t25353 * t2752;
    let t86843 = t6547 * t25213;
    let t86844 = F::cast_from(0.38381794893125283518e-1_f64) * t86843;
    let t86849 = t857 * t4119;
    let t86868 = t23168 * t25342;
    (t86730, t86736, t86753, t86836, t86844, t86849, t86868)
}
