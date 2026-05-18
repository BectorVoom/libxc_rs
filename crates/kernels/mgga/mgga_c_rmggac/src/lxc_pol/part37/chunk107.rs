//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 107/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk107<F: Float>(t53: F, t60: F, t431: F, t433: F, t195: F, t231: F, t57: F, t280: F, t62: F, t284: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t435 = F::new(0.5848223622634646207e0) * t431 * t433;
    let t436 = t195 * t231;
    let t437 = F::new(1.0) / t57;
    let t440 = piecewise3::<f64>(t54, F::new(0.0), F::new(2.0) / F::new(3.0) * t437 * t280);
    let t441 = F::new(1.0) / t62;
    let t444 = piecewise3::<f64>(t61, F::new(0.0), F::new(2.0) / F::new(3.0) * t441 * t284);
    let t446 = t440 / F::new(2.0) + t444 / F::new(2.0);
    (t435, t436, t437, t441, t446)
}
