//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1137/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1137<F: Float>(t26083: F, t9239: F, t45844: F, t6489: F, t22716: F, t7697: F, t7692: F, t81186: F, t1834: F, t794: F, t26197: F, t80670: F, t213: F, t225: F, t22724: F, t26474: F) -> (F, F, F, F, F, F, F, F) {
    let t90192 = t9239 * t26083;
    let t90330 = t45844 * t6489;
    let t90503 = t22716 * t7697;
    let t90521 = t81186 * t7692;
    let t90544 = t794 * t1834;
    let t90551 = t80670 * t26197;
    let t90566 = t213 * t1834 * t225;
    let t90582 = t22724 * t26474;
    (t90192, t90330, t90503, t90521, t90544, t90551, t90566, t90582)
}
