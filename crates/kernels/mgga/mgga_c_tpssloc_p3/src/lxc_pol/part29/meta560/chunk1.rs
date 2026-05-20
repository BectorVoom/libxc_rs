//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1964/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1964<F: Float>(t2121: F, t2136: F, t24650: F, t24747: F, t24752: F, t24754: F, t27681: F, t27684: F, t27687: F, t27692: F, t27697: F, t27701: F, t27704: F, t27711: F, t27714: F, t4989: F, t7321: F, t7326: F, t7331: F, t7345: F, t8040: F) -> F {
    let t27719 = -F::cast_from(0.80745512188280781712e-3_f64) * t27681 - F::cast_from(0.10093189023535097714e-3_f64) * t27684 * t7331 - t2121 * t27687 / F::new(144.0) + F::cast_from(0.10093189023535097714e-3_f64) * t7326 * t27692 - F::cast_from(0.10093189023535097714e-3_f64) * t24650 * t8040 + t2121 * t27697 / F::new(216.0) + F::cast_from(0.10093189023535097714e-3_f64) * t27701 - F::cast_from(0.10093189023535097714e-3_f64) * t27704 * t7321 + F::new(5.0) / F::new(6912.0) * t7345 * t4989 - F::cast_from(0.10093189023535097714e-3_f64) * t24747 - F::cast_from(0.80745512188280781712e-3_f64) * t27711 * t7331 - F::cast_from(0.10093189023535097714e-3_f64) * t27714 * t2136 - t24752 / F::new(3456.0) + t24754 / F::new(2304.0);
    t27719
}
