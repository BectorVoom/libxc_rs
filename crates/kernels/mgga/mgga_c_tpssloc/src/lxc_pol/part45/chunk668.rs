//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 668/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk668<F: Float>(t25: F, t28: F, t265: F, t394: F, t504: F, t1914: F, t202: F, t8565: F, t1877: F, t193: F, t7114: F, t870: F, t40: F, t8566: F, t52: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t8569 = t25 * t1914;
    let t8574 = t202 * t8565;
    let t8579 = -t1877 * t1914 * t7114 + t193 * t8574 * t870;
    let t8580 = piecewise3::<f64>(t395, F::new(0.0), t8579);
    let t8583 = piecewise3::<f64>(t115, t1877 * t8566 * t25 / F::new(2.0) - t1877 * t7114 * t8569 / F::new(2.0), t8580 * t40 / F::new(2.0));
    let t8586 = t28 * t1914;
    let t8591 = piecewise3::<f64>(t505, F::new(0.0), t8579);
    let t8594 = piecewise3::<f64>(t401, t1877 * t8566 * t28 / F::new(2.0) - t1877 * t7114 * t8586 / F::new(2.0), t8591 * t52 / F::new(2.0));
    (t8569, t8580, t8583, t8586, t8591, t8594)
}
