//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 176/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk176<F: Float>(t53: F, t60: F, t155: F, t578: F, t181: F, t577: F, t437: F, t521: F, t441: F, t525: F, zeta_threshold: F) -> (F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t579 = t155 * t578;
    let t581 = F::cast_from(0.19751673498613801407e-1_f64) * t577 * t181;
    let t584 = piecewise3::<F>(t54, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t437 * t521);
    let t587 = piecewise3::<F>(t61, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t441 * t525);
    let t589 = t584 / F::cast_from(2.0_f64) + t587 / F::cast_from(2.0_f64);
    (t579, t581, t589)
}
