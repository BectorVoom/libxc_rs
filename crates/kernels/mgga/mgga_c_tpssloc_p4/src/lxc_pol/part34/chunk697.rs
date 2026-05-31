//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 697/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk697<F: Float>(t28: F, t265: F, t504: F, t2057: F, t7649: F, t7864: F, t1409: F, t1649: F, t1877: F, t2071: F, t2522: F, t52: F, t7114: F, t7656: F, t7845: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t7871 = t2057 * t7649;
    let t7884 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t7864);
    let t7889 = piecewise3::<F>(t401, F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7871 + t1877 * t7845 * t28 / F::cast_from(2.0_f64) - t1877 * t7114 * t7656 / F::cast_from(2.0_f64) + t1877 * t2057 * t1649 / F::cast_from(2.0_f64), -t2071 * t1409 / F::cast_from(2.0_f64) + t7884 * t52 / F::cast_from(2.0_f64));
    (t7884, t7889)
}
