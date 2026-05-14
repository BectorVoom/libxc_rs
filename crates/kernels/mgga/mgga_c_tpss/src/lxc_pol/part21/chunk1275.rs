//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1275/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1275<F: Float>(t12679: F, t18547: F, t24790: F, t1163: F, t19596: F, t626: F, t19597: F, t3499: F, t3166: F, t6112: F, t17907: F, t6103: F, t13133: F, t5532: F, t1689: F, t42336: F) -> (F, F, F, F, F, F, F) {
    let t63715 = 6.0 * t18547 * t24790 * t12679;
    let t63718 = 4.0 * t626 * t1163 * t19596;
    let t63725 = 4.0 * t3499 * t19597;
    let t63728 = 2.0 * t626 * t3166 * t6112;
    let t63730 = 2.0 * t6103 * t17907;
    let t63740 = 4.0 * t13133 * t5532;
    let t63742 = 2.0 * t42336 * t1689;
    (t63715, t63718, t63725, t63728, t63730, t63740, t63742)
}
