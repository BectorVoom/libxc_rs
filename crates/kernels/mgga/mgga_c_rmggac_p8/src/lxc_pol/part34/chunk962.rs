//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 962/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk962<F: Float>(t2080: F, t739: F, t9530: F, t15439: F, t2604: F, t68582: F, t68602: F, t74462: F, t74468: F, t74471: F, t74477: F, t74491: F, t77014: F, t77015: F, t77017: F, t77020: F, t77023: F, t77026: F, t77031: F, t77034: F) -> F {
    let t77036 = t739 * t9530 * t2080;
    let t77037 = F::cast_from(0.2993560425465952141e-1_f64) * t77036;
    let t77038 = -t77014 + t77015 + F::cast_from(0.35038612185802734376e-6_f64) * t74462 - t77017 + t74468 - t77020 + t77023 + t77026 + F::cast_from(0.72714524817717142308e-5_f64) * t74471 + F::cast_from(0.70077224371605468752e-6_f64) * t74477 - F::cast_from(0.40878380883436523436e-5_f64) * t68582 - F::cast_from(0.81756761766873046877e-6_f64) * t68602 + t77031 - F::cast_from(0.59871208509319042821e-1_f64) * t2604 * t15439 + t77034 - t77037 - t74491;
    t77038
}
