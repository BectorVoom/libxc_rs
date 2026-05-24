//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 826/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk826<F: Float>(t118: F, t7417: F, t16503: F, t2281: F, t7461: F, t1357: F, t34976: F, t7448: F, t34975: F, t35039: F, t7455: F, t34761: F, t9165: F) -> (F, F, F, F) {
    let t38508 = t7417 * t118;
    let t38511 = t16503 * t38508 * t2281 * t7461;
    let t38515 = t16503 * t34976 * t1357 * t7448;
    let t38519 = t34975 * t35039 * t2281 * t7455;
    let t38521 = t34761 * t9165;
    (t38511, t38515, t38519, t38521)
}
