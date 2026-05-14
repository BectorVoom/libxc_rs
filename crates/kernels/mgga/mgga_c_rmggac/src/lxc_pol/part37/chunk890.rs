//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 890/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk890<F: Float>(t14980: F, t1652: F, t1356: F, t68582: F, t68602: F, t74439: F, t74442: F, t74462: F, t74468: F, t74471: F, t74477: F, t77011: F, t77012: F, t77014: F, t77015: F, t77017: F, t77020: F, t77023: F, t77026: F) -> (F, F) {
    let t80102 = t14980 * t1652;
    let t80109 = -0.35038612185802734374e-6 * t74439 - 0.87596530464506835935e-6 * t74442 + t77011 - t77012 - t77014 + t77015 + 0.35038612185802734374e-6 * t74462 - t77017 + t74468 - t77020 + t77023 + t77026 + 0.39914139006212695214e-1 * t1356 * t80102 + 0.72714524817717142305e-5 * t74471 + 0.70077224371605468748e-6 * t74477 - 0.40878380883436523435e-5 * t68582 - 0.81756761766873046873e-6 * t68602;
    (t80102, t80109)
}
