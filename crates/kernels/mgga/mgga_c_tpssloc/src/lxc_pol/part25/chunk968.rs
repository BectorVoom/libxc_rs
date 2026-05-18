//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 968/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk968<F: Float>(t12098: F, t12101: F, t12103: F, t12105: F, t12107: F, t12109: F, t12112: F, t12114: F, t12116: F, t12118: F, t12121: F, t12123: F, t9797: F, t9820: F, t9824: F) -> F {
    let t12476 = t9797 - t9820 - t9824 + t12098 - t12101 + t12103 - t12105 + t12107 - t12109 + t12112 - t12114 + t12116 + t12118 + t12121 + t12123;
    t12476
}
