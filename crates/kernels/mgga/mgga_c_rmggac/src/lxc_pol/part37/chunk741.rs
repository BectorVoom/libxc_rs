//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 741/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk741<F: Float>(t69936: F, t69938: F, t69940: F, t69942: F, t14696: F, t7335: F, t2019: F, t3180: F, t7926: F, t14572: F, t7487: F, t14559: F, t2020: F) -> (F, F, F, F, F, F, F, F) {
    let t71545 = F::new(0.32526727992809621482e-4) * t69936;
    let t71546 = F::new(0.60975299583150056624e-3) * t69938;
    let t71551 = F::new(0.16263363996404810741e-4) * t69940;
    let t71552 = F::new(0.16263363996404810741e-4) * t69942;
    let t71564 = t7335 * t14696;
    let t71565 = F::new(0.15243824895787514157e-3) * t71564;
    let t71581 = t2019 * t7926 * t3180;
    let t71582 = F::new(0.81300399444200075504e-3) * t71581;
    let t71583 = t7487 * t14572;
    let t71594 = t2019 * t2020 * t14559;
    (t71545, t71546, t71551, t71552, t71565, t71582, t71583, t71594)
}
