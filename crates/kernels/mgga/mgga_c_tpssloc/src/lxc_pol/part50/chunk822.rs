//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 822/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk822<F: Float>(t28: F, t265: F, t504: F, t1877: F, t8366: F, t8370: F, t8424: F, t52: F, t8428: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t8434 = t1877 * t8366 * t28 / F::new(2.0) - t1877 * t8370 * t28 / F::new(2.0);
    let t8435 = piecewise3::<f64>(t505, F::new(0.0), t8424);
    let t8438 = piecewise3::<f64>(t401, t8434, t8435 * t52 / F::new(2.0));
    let t8439 = t8428 + t8438;
    (t8435, t8439)
}
