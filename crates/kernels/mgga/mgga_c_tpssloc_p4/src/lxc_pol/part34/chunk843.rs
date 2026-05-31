//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 843/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk843<F: Float>(t19682: F, t15972: F, t12094: F, t12103: F, t12105: F, t12109: F, t12114: F, t12116: F, t9793: F, t9797: F, t9820: F, t9824: F) -> (F, F, F) {
    let t20523 = F::cast_from(0.17544670867903938621e1_f64) * t19682;
    let t20524 = F::cast_from(3.0_f64) * t15972;
    let t20525 = -t12094 + t9793 + t9797 - t9820 - t9824 - t20523 + t20524 + t12103 - t12105 - t12109 - t12114 + t12116;
    (t20523, t20524, t20525)
}
