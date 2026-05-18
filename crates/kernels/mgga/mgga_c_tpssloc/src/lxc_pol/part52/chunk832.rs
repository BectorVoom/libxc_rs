//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 832/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk832<F: Float>(t25: F, t1409: F, t1965: F, t40: F, t7552: F, t7643: F, t1484: F, t28: F, t1915: F, t1530: F, t1649: F, t1877: F, t2522: F, t6670: F, t7541: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t7648 = piecewise3::<f64>(t115, t7552, t1965 * t1409 / F::new(2.0) + t7643 * t40 / F::new(2.0));
    let t7649 = t28 * t1484;
    let t7650 = t1915 * t7649;
    let t7656 = t28 * t1530;
    let t7663 = F::new(3.0) / F::new(2.0) * t2522 * t7650 + t1877 * t7541 * t28 / F::new(2.0) - t1877 * t6670 * t7656 / F::new(2.0) + t1877 * t1915 * t1649 / F::new(2.0);
    (t7648, t7649, t7656, t7663)
}
