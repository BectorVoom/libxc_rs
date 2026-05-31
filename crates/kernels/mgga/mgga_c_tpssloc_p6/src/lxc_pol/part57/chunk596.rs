//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 596/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk596<F: Float>(t28: F, t265: F, t504: F, t1649: F, t1877: F, t1915: F, t2522: F, t6670: F, t7541: F, t7650: F, t7656: F, t7642: F, t1409: F, t1972: F, t52: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t7663 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7650 + t1877 * t7541 * t28 / F::cast_from(2.0_f64) - t1877 * t6670 * t7656 / F::cast_from(2.0_f64) + t1877 * t1915 * t1649 / F::cast_from(2.0_f64);
    let t7664 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t7642);
    let t7669 = piecewise3::<F>(t401, t7663, -t1972 * t1409 / F::cast_from(2.0_f64) + t7664 * t52 / F::cast_from(2.0_f64));
    (t7664, t7669)
}
