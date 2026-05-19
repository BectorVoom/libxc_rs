//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 743/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk743<F: Float>(t69934: F, t69936: F, t69938: F, t69940: F, t69942: F, t14696: F, t7335: F, t2019: F, t3180: F, t7926: F, t14572: F, t7487: F) -> (F, F, F, F, F, F, F, F) {
    let t71544 = F::cast_from(0.30487649791575028312e-3_f64) * t69934;
    let t71545 = F::cast_from(0.32526727992809621482e-4_f64) * t69936;
    let t71546 = F::cast_from(0.60975299583150056624e-3_f64) * t69938;
    let t71551 = F::cast_from(0.16263363996404810741e-4_f64) * t69940;
    let t71552 = F::cast_from(0.16263363996404810741e-4_f64) * t69942;
    let t71564 = t7335 * t14696;
    let t71565 = F::cast_from(0.15243824895787514157e-3_f64) * t71564;
    let t71581 = t2019 * t7926 * t3180;
    let t71582 = F::cast_from(0.81300399444200075504e-3_f64) * t71581;
    let t71583 = t7487 * t14572;
    (t71544, t71545, t71546, t71551, t71552, t71565, t71582, t71583)
}
