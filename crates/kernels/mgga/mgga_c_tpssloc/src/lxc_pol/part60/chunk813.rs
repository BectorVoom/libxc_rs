//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 813/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk813<F: Float>(t28: F, t265: F, t504: F, t29148: F, t1409: F, t2071: F, t29188: F, t52: F, t5398: F, t7884: F, t29156: F, t5161: F, t7940: F, t1458: F, t7890: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t29189 = piecewise3::<F>(t505, F::new(0.0), t29148);
    let t29196 = piecewise3::<F>(t401, t29188, t29189 * t52 / F::new(2.0) - t7884 * t1409 - t2071 * t5398 / F::new(2.0));
    let t29197 = t29156 + t29196;
    let t29201 = t7940 * t5161;
    let t29205 = t7890 * t1458;
    (t29197, t29201, t29205)
}
