//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 481/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk481<F: Float>(t1687: F, t300: F, t1694: F, t3375: F, t1171: F, t1706: F, t1420: F, t972: F) -> (F, F, F, F) {
    let t4869 = t300 * t1687;
    let t4874 = t3375 * t1694;
    let t4887 = t1706 * t1171;
    let t4889 = t1420 * t972;
    (t4869, t4874, t4887, t4889)
}
