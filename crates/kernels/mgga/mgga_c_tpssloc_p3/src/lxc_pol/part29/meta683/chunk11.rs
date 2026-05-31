//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2325/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2325<F: Float>(t27710: F, t3: F, t24684: F, t15608: F, t24741: F, t11716: F, t1210: F, t14744: F, t1714: F, t1734: F, t2121: F, t2132: F, t24699: F, t27636: F, t27637: F, t27642: F, t27644: F, t27704: F, t3448: F, t3507: F, t475: F, t4950: F, t5011: F, t6729: F, t7321: F, t7331: F, t8040: F, t85827: F, t85966: F, t85972: F, t86194: F, t86330: F, t86357: F, t95396: F) -> F {
    let t95648 = t27710 * t3;
    let t95649 = t95648 * t24684;
    let t95662 = t24741 * t15608 / F::cast_from(1728.0_f64);
    let t95672 = F::cast_from(0.60559134141210586284e-3_f64) * t95396 * t11716 * t1734 * t85966 * t3507 - F::cast_from(0.60559134141210586284e-3_f64) * t95396 * t27637 * t85972 * t3507 - F::cast_from(0.10093189023535097714e-3_f64) * t27636 * t27642 * t85827 * t475 + F::cast_from(0.16149102437656156342e-2_f64) * t95649 * t7331 - F::cast_from(0.20186378047070195428e-3_f64) * t27636 * t1210 * t5011 * t27644 + F::cast_from(0.20186378047070195428e-3_f64) * t86194 * t8040 - F::cast_from(0.10093189023535097714e-3_f64) * t86357 - t86330 * t4950 / F::cast_from(1152.0_f64) - t95662 - t2121 * t3448 * t14744 / F::cast_from(48.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t27704 * t24699 + F::cast_from(0.20186378047070195428e-3_f64) * t2132 * t6729 * t1714 * t7321;
    t95672
}
