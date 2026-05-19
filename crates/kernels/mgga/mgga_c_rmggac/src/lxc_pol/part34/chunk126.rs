//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 126/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk126<F: Float>(t53: F, t60: F, t521: F, t57: F, t62: F, zeta_threshold: F) -> (F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t524 = piecewise3::<F>(t54, F::new(0.0), F::new(4.0) / F::new(3.0) * t57 * t521);
    let t525 = -t521;
    let t528 = piecewise3::<F>(t61, F::new(0.0), F::new(4.0) / F::new(3.0) * t62 * t525);
    let t529 = t524 + t528;
    (t525, t529)
}
