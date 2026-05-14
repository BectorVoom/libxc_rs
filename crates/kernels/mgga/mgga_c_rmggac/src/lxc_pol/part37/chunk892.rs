//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 892/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk892<F: Float>(t74523: F, t14980: F, t1550: F, t1624: F, t1627: F, t3282: F, t739: F, t74508: F, t74511: F, t74514: F, t74517: F, t74520: F, t77069: F, t77070: F, t77075: F, t77077: F, t77081: F, t77082: F, t77083: F, t77084: F, t8377: F, t903: F) -> (F,) {
    let t80118 = 0.82834157616596963771e-1 * t74523;
    let t80128 = -0.32526727992809621482e-5 * t74508 - 0.32526727992809621482e-5 * t74511 - 0.32526727992809621482e-5 * t74514 - 0.32526727992809621482e-5 * t74517 + t74520 + t80118 - t77069 + t77070 - t77075 - t77077 - t77081 + t77082 - t77083 - 0.11974241701863808564e0 * t1550 * t3282 * t1624 + 0.17961362552795712846e0 * t903 * t3282 * t1627 + 0.11974241701863808564e0 * t739 * t14980 * t8377 + t77084;
    (t80128,)
}
