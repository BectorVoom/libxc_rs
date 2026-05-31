//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 332/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk332<F: Float>(t1000: F, t1005: F, t1020: F, t1025: F, t1032: F, t1038: F, t1041: F, t1046: F, t350: F, t378: F, t964: F, t973: F, t997: F) -> F {
    let t1049 = -t964 * t350 / F::cast_from(36.0_f64) + t997 + t973 * t1000 / F::cast_from(288.0_f64) + t1005 * t378 / F::cast_from(3072.0_f64) + t1020 * t1025 / F::cast_from(3072.0_f64) - t1032 * t378 / F::cast_from(576.0_f64) + t1038 + t1041 * t1046 / F::cast_from(4608.0_f64);
    t1049
}
