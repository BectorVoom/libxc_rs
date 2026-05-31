//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2319/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2319<F: Float>(t15689: F, t7310: F, t27674: F, t3548: F, t15753: F, t27608: F, t7321: F, t1222: F, t27586: F, t15357: F, t15560: F, t2134: F, t24650: F, t27580: F, t27692: F, t27714: F, t460: F, t7320: F, t8040: F, t86282: F, t86296: F, t86324: F) -> F {
    let t95507 = t7310 * t15689 / F::cast_from(432.0_f64);
    let t95511 = t27674 * t3548 / F::cast_from(162.0_f64);
    let t95512 = t7310 * t15753;
    let t95515 = F::cast_from(0.20186378047070195428e-3_f64) * t27608 * t7321;
    let t95517 = t27586 * t1222 / F::cast_from(1152.0_f64);
    let t95518 = -F::cast_from(0.20186378047070195428e-3_f64) * t24650 * t27692 - F::cast_from(0.10093189023535097714e-3_f64) * t86296 * t8040 + F::cast_from(0.20186378047070195428e-3_f64) * t27714 * t7321 - F::cast_from(0.10093189023535097714e-3_f64) * t2134 * t15357 * t460 * t7320 - t86324 * t15560 / F::cast_from(1152.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t86282 - t95507 - F::cast_from(0.16149102437656156342e-2_f64) * t27580 * t7321 + t95511 + t95512 / F::cast_from(1296.0_f64) - t95515 + t95517;
    t95518
}
