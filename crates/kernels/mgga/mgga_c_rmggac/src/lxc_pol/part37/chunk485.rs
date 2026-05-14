//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 485/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk485<F: Float>(t26: F, t498: F, t2067: F, t14237: F, t14236: F, t1985: F, t797: F) -> (F, F, F) {
    let t14238 = t26 * t498;
    let t14239 = t2067 * t14238;
    let t14240 = t14237 * t14239;
    let t14241 = t14236 * t14240;
    let t14243 = t1985 * t797;
    (t14240, t14241, t14243)
}
