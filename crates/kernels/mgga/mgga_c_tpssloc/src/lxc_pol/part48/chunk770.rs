//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 770/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk770<F: Float>(t24098: F, t24164: F, t533: F, t1390: F, t2095: F, t23857: F, t532: F, t7216: F, t6879: F, t193: F, t201: F, t2056: F) -> (F, F, F, F, F) {
    let t24165 = t24098 + t24164;
    let t24166 = t533 * t24165;
    let t24167 = t24166 * t1390;
    let t24169 = t2095 * t23857;
    let t24175 = t532 * t7216;
    let t24176 = t24175 * t6879;
    let t24191 = t193 * t201 * t2056;
    (t24166, t24167, t24169, t24176, t24191)
}
