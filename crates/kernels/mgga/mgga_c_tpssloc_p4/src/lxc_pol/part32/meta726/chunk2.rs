//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2343/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2343<F: Float>(t28: F, t265: F, t504: F, t100624: F, t104708: F, t100805: F, t1409: F, t16558: F, t2161: F, t27850: F, t29840: F, t3966: F, t52: F, t5398: F, t607: F, t7402: F, t8097: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t104709 = piecewise3::<F>(t505, t104708, t100624);
    let t104721 = piecewise3::<F>(t401, t100805, t104709 * t52 / F::new(2.0) - t29840 * t607 / F::new(2.0) - t27850 * t1409 - t8097 * t3966 - t7402 * t5398 / F::new(2.0) - t2161 * t16558 / F::new(2.0));
    t104721
}
