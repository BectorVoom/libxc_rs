//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1317/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1317<F: Float>(t17930: F, t52613: F, t4802: F, t750: F, t1364: F, t555: F, t63783: F, t4578: F, t821: F, t1398: F, t3724: F, t19817: F) -> (F, F, F, F, F, F, F) {
    let t69858 = t17930 * t52613;
    let t69863 = t4802 * t750;
    let t69864 = t17930 * t69863;
    let t69868 = t63783 * t555 * t1364;
    let t69871 = t4578 * t821;
    let t69881 = t1398 * t3724;
    let t69882 = t19817 * t69881;
    (t69858, t69863, t69864, t69868, t69871, t69881, t69882)
}
