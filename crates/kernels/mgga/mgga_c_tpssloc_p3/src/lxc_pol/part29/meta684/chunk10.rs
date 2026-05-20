//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2336/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2336<F: Float>(t28: F, t265: F, t504: F, t89823: F, t95952: F, t12606: F, t1409: F, t2161: F, t2250: F, t24916: F, t27850: F, t3966: F, t52: F, t607: F, t7402: F, t8097: F, t90003: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t95953 = piecewise3::<F>(t505, t95952, t89823);
    let t95965 = piecewise3::<F>(t401, t90003, t95953 * t52 / F::new(2.0) - t27850 * t607 - t8097 * t2250 / F::new(2.0) - t24916 * t1409 / F::new(2.0) - t7402 * t3966 - t2161 * t12606 / F::new(2.0));
    t95965
}
