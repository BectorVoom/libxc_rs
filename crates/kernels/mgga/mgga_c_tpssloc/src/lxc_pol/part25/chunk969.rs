//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 969/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk969<F: Float>(t1887: F, t22839: F, t552: F, t6604: F, t12461: F, t2094: F, t193: F, t200: F, t2056: F, t10109: F, t2053: F, t2061: F, t2035: F, t671: F, t12020: F, t2091: F) -> (F, F, F, F, F, F, F, F) {
    let t26331 = t22839 * t1887;
    let t26446 = t6604 * t552;
    let t26558 = t2094 * t12461;
    let t26563 = t193 * t200 * t2056;
    let t26728 = t10109 * t2053;
    let t26756 = t193 * t2061;
    let t26977 = t2035 * t671;
    let t26989 = t12020 * t2091;
    (t26331, t26446, t26558, t26563, t26728, t26756, t26977, t26989)
}
