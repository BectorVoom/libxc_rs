//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1223/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1223<F: Float>(t28: F, t265: F, t504: F, t1254: F, t1256: F, t193: F, t336: F, t4700: F, t6834: F, t7394: F, t7398: F, t2161: F, t52: F, t607: F, t6855: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t7402 = piecewise3::<F>(t505, t1256 * t193 * t336 * t7394 - t1254 * t4700 * t7398, t6834);
    let t7407 = piecewise3::<F>(t401, t6855, -t2161 * t607 / F::new(2.0) + t7402 * t52 / F::new(2.0));
    (t7402, t7407)
}
