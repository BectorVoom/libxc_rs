//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 701/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk701<F: Float>(t69618: F, t8450: F, t69621: F, t14225: F, t3352: F, t9164: F, t3140: F, t8675: F, t13868: F, t11683: F, t14236: F, t14243: F, t2078: F, t2808: F, t511: F, t14258: F) -> (F, F, F, F, F) {
    let t74535 = t8450 * t69618;
    let t74536 = t74535 * t69621;
    let t74539 = t14225 * t3352 * t9164;
    let t74548 = t8675 * t3140;
    let t74549 = t74548 * t13868;
    let t74553 = t14236 * t14243 * t2078 * t11683;
    let t74555 = t511 * t2808;
    let t74556 = t3352 * t74555;
    let t74557 = t14258 * t74556;
    (t74536, t74539, t74549, t74553, t74557)
}
