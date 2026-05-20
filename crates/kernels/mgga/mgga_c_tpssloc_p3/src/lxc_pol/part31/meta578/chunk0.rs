//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1814/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1814<F: Float>(t26339: F, t81159: F, t22716: F, t7697: F, t26216: F, t26210: F, t6897: F, t794: F, t1377: F, t5187: F, t7692: F, t81186: F) -> (F, F, F, F, F, F) {
    let t90500 = t81159 * t26339;
    let t90503 = t22716 * t7697;
    let t90511 = t81159 * t26216;
    let t90514 = t6897 * t794 * t26210;
    let t90516 = t1377 * t5187;
    let t90521 = t81186 * t7692;
    (t90500, t90503, t90511, t90514, t90516, t90521)
}
