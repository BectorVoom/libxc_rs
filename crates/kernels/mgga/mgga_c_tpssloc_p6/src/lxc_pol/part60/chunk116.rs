//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 116/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk116<F: Float>(t360: F, t34: F, t35: F, rho0: F, sigma0: F) -> (F, F, F, F, F) {
    let t361 = t360 - F::new(1.0);
    let t362 = F::new(1.0) / t361;
    let t363 = sigma0 * sigma0;
    let t364 = t362 * t363;
    let t365 = t34 * t34;
    let t366 = t365 * rho0;
    let t368 = F::new(1.0) / t35 / t366;
    (t361, t362, t363, t364, t368)
}
