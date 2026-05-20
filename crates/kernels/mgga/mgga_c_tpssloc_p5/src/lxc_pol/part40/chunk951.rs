//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 951/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk951<F: Float>(t1174: F, t11835: F, t10471: F, t11715: F, t11712: F, t11721: F, t6739: F, t3502: F, t3508: F, t11707: F, t3609: F, t3623: F) -> (F, F, F, F, F, F, F) {
    let t11836 = t1174 * t11835;
    let t11880 = t10471 * t11715;
    let t11881 = t11712 * t11880;
    let t11883 = t6739 * t11721;
    let t11887 = t10471 * t3502;
    let t11888 = t11712 * t11887;
    let t11889 = t6739 * t3508;
    let t11904 = t11707 * t3609;
    let t11907 = t11707 * t3623;
    (t11836, t11881, t11883, t11888, t11889, t11904, t11907)
}
