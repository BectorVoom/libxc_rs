//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 997/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk997<F: Float>(t10471: F, t3502: F, t11712: F, t3508: F, t6739: F, t11707: F, t3609: F, t3623: F, t1209: F, t225: F, t3591: F, t3482: F) -> (F, F, F, F, F, F, F) {
    let t11887 = t10471 * t3502;
    let t11888 = t11712 * t11887;
    let t11889 = t6739 * t3508;
    let t11904 = t11707 * t3609;
    let t11907 = t11707 * t3623;
    let t11913 = t10471 * t1209;
    let t11914 = t11712 * t11913;
    let t11925 = t3591 * t225;
    let t11928 = t3482 * t225;
    (t11888, t11889, t11904, t11907, t11914, t11925, t11928)
}
