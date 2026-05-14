//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 712/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk712<F: Float>(t35688: F, t70171: F, t9081: F, t11674: F, t498: F, t14236: F, t2067: F, t69629: F, t11675: F, t68427: F, t11679: F, t70397: F, t13806: F, t8368: F, t15379: F, t68937: F) -> (F, F, F, F, F, F) {
    let t74725 = t35688 * t70171 * t9081;
    let t74727 = t11674 * t498;
    let t74730 = t14236 * t69629 * t2067 * t74727;
    let t74734 = t14236 * t68427 * t2067 * t11675;
    let t74739 = t14236 * t70397 * t2067 * t11679;
    let t74741 = t8368 * t13806;
    let t74743 = t15379 * t68937;
    (t74725, t74730, t74734, t74739, t74741, t74743)
}
