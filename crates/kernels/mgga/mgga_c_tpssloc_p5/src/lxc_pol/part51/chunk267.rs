//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 267/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk267<F: Float>(t270: F, t283: F, t61: F, t248: F, t884: F, t1000: F, t1005: F, t1020: F, t1025: F, t1032: F, t1038: F, t1041: F, t350: F, t378: F, t964: F, t973: F, t997: F) -> (F, F, F, F) {
    let t1043 = F::new(1.0) / t283 / t270;
    let t1044 = t61 * t1043;
    let t1046 = t248 * t1044 * t884;
    let t1049 = -t964 * t350 / F::new(36.0) + t997 + t973 * t1000 / F::new(288.0) + t1005 * t378 / F::new(3072.0) + t1020 * t1025 / F::new(3072.0) - t1032 * t378 / F::new(576.0) + t1038 + t1041 * t1046 / F::new(4608.0);
    (t1043, t1044, t1046, t1049)
}
