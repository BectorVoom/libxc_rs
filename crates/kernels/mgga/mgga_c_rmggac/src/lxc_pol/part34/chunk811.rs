//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 811/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk811<F: Float>(t2841: F, t498: F, t14236: F, t2067: F, t69588: F, t13848: F, t13850: F, t8608: F, t13858: F, t2412: F, t15220: F, t2191: F) -> (F, F, F, F) {
    let t74571 = t2841 * t498;
    let t74574 = t14236 * t69588 * t2067 * t74571;
    let t74577 = t8608 * t13848 * t13850;
    let t74579 = t2412 * t13858;
    let t74581 = t2191 * t15220;
    (t74574, t74577, t74579, t74581)
}
