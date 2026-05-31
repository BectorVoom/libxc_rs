//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2738/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2738<F: Float>(t1799: F, t3698: F, t20063: F, t3701: F, t1388: F, t15899: F, t3918: F, t39642: F, t39655: F, t39658: F, t5160: F, t57206: F, t57207: F, t57209: F, t57210: F, t57212: F, t57213: F, t57214: F) -> F {
    let t57802 = t1799 * t3698;
    let t57806 = t20063 * t3701;
    let t57810 = -F::cast_from(2.0_f64) * t1388 * t5160 * t57806 + F::cast_from(12.0_f64) * t15899 * t3918 * t57802 + t39642 - t39655 - t39658 + t57206 + t57207 + t57209 + t57210 + t57212 - t57213 + t57214;
    t57810
}
