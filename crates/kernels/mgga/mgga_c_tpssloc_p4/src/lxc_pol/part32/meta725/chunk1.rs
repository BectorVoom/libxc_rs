//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2330/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2330<F: Float>(t104355: F, t104364: F, t104367: F, t104369: F, t104371: F, t104375: F, t18215: F, t2121: F, t2132: F, t2133: F, t24736: F, t27703: F, t4899: F, t6138: F, t6203: F, t7321: F, t8027: F, t95540: F, t95542: F, t95545: F) -> F {
    let t104380 = F::cast_from(0.10093189023535097714e-3_f64) * t104355 - F::cast_from(0.10093189023535097714e-3_f64) * t2132 * t2133 * t6138 * t7321 + t2121 * t4899 * t18215 / F::new(108.0) - F::cast_from(0.20186378047070195428e-3_f64) * t104364 - F::cast_from(0.10093189023535097714e-3_f64) * t104367 - t104369 / F::new(3456.0) - t104371 / F::new(1728.0) + F::new(5.0) / F::new(6912.0) * t24736 * t6203 - t104375 / F::new(1728.0) + F::cast_from(0.16149102437656156342e-2_f64) * t8027 * t27703 * t7321 - t95540 + t95542 + t95545;
    t104380
}
