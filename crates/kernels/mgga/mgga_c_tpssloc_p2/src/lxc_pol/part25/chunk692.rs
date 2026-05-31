//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 692/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk692<F: Float>(t28: F, t265: F, t504: F, t7130: F, t1081: F, t1877: F, t2057: F, t2071: F, t2522: F, t52: F, t607: F, t6841: F, t6848: F, t7110: F, t7114: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t7150 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t7130);
    let t7155 = piecewise3::<F>(t401, F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t6841 + t1877 * t7110 * t28 / F::cast_from(2.0_f64) - t1877 * t7114 * t6848 / F::cast_from(2.0_f64) + t1877 * t2057 * t1081 / F::cast_from(2.0_f64), -t2071 * t607 / F::cast_from(2.0_f64) + t7150 * t52 / F::cast_from(2.0_f64));
    (t7150, t7155)
}
