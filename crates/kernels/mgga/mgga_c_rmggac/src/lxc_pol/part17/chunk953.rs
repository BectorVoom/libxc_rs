//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 953/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk953<F: Float>(t1364: F, t1635: F, t2024: F, t2402: F, t30311: F, t35327: F, t39786: F, t39789: F, t39792: F, t39797: F, t39801: F, t39804: F, t39809: F, t39827: F, t45798: F, t45811: F, t45813: F, t45818: F, t45822: F, t5898: F, t8800: F, t884: F) -> F {
    let t45824 = t45798 - F::cast_from(0.33105799917009430643e-4_f64) * t35327 - t39786 - F::cast_from(0.30487649791575028314e-3_f64) * t39789 - F::cast_from(0.3903207359137154578e-3_f64) * t39792 - t39797 - t39801 - F::cast_from(0.30487649791575028314e-3_f64) * t39804 + t39809 - F::cast_from(0.47896966807455234256e0_f64) * t1364 * t2402 * t1635 - F::cast_from(0.23948483403727617128e0_f64) * t884 * t8800 * t5898 - F::cast_from(0.25538759935978703638e-4_f64) * t45811 + F::cast_from(0.25538759935978703638e-4_f64) * t45813 - F::cast_from(0.11974241701863808564e0_f64) * t884 * t2024 * t30311 - t39827 + F::cast_from(0.85129199786595678796e-5_f64) * t45818 + F::cast_from(0.76616279807936110914e-4_f64) * t45822;
    t45824
}
