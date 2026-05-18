//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 819/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk819<F: Float>(t577: F, t669: F, t7933: F, t7934: F, t35688: F, t70171: F, t9081: F, t11674: F, t498: F, t14236: F, t2067: F, t69629: F) -> (F, F, F) {
    let t74722 = t7933 * t7934 * t577 * t669;
    let t74725 = t35688 * t70171 * t9081;
    let t74727 = t11674 * t498;
    let t74730 = t14236 * t69629 * t2067 * t74727;
    (t74722, t74725, t74730)
}
