//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 726/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk726<F: Float>(t14116: F, t14117: F, t9164: F, t3154: F, t9087: F, t14371: F, t15336: F, t14027: F, t15340: F, t70554: F, t1550: F, t2060: F, t41091: F, t41006: F, t903: F, t13962: F, t3056: F, t8475: F) -> (F, F, F, F, F, F, F) {
    let t75054 = t14116 * t14117 * t9164;
    let t75060 = t9087 * t3154;
    let t75062 = t14371 * t15336;
    let t75065 = t15340 * t70554 * t14027;
    let t75069 = 0.5987120850931904282e-1 * t1550 * t2060 * t41091;
    let t75072 = 0.8980681276397856423e-1 * t903 * t2060 * t41006;
    let t75074 = t3056 * t13962 * t8475;
    (t75054, t75060, t75062, t75065, t75069, t75072, t75074)
}
