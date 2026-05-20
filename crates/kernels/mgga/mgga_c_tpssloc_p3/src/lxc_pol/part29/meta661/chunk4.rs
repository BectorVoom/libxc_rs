//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2199/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2199<F: Float>(t12030: F, t12444: F, t1375: F, t22630: F, t26226: F, t26482: F, t3752: F, t3758: F, t3887: F, t3911: F, t5321: F, t568: F, t7722: F, t7729: F, t7749: F, t81264: F, t90659: F, t90663: F, t90665: F, t90670: F) -> F {
    let t90677 = -F::new(6.0) * t5321 * t22630 + F::cast_from(0.52089578783527170488e-1_f64) * t81264 + F::new(2.0) * t12030 * t7729 + F::new(4.0) * t12444 * t7729 - F::cast_from(0.63969658155208805863e-1_f64) * t90659 - F::cast_from(0.82246703342411321824e-2_f64) * t90663 - F::new(12.0) * t90665 * t26226 + F::new(4.0) * t3758 * t26482 + t90670 + F::new(2.0) * t1375 * t3887 * t7749 * t3911 + t3752 * t7722 * t568;
    t90677
}
