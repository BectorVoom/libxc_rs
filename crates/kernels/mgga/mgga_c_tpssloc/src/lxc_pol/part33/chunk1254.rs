//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1254/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1254<F: Float>(t12250: F, t1992: F, t74967: F, t81027: F, t22897: F, t3792: F, t107397: F, t107402: F, t107406: F, t107413: F, t107417: F, t107431: F, t1336: F, t1814: F, t2013: F, t20595: F, t26403: F, t28152: F, t28156: F, t28178: F, t5234: F, t5344: F, t6388: F, t6415: F, t91029: F, t91078: F, t91081: F, t97179: F, t97200: F, t97494: F) -> (F,) {
    let t107435 = t1992 * t81027 * t74967 * t12250;
    let t107439 = t1992 * t22897 * t74967 * t3792;
    let t107442 = 6.0 * t1336 * t91029 * t6388 - 0.24674011002723396548e-1 * t107397 - 0.34543615403812755166e0 * t97179 + 0.82246703342411321825e-2 * t107402 - 0.82246703342411321825e-2 * t107406 + 3.0 * t1814 * t28156 - 0.57572692339687925277e-1 * t97200 - 0.24674011002723396548e-1 * t107413 + 0.14804406601634037928e0 * t107417 - 3.0 * t5234 * t28152 - 0.78134368175290755733e-1 * t91078 - 3.0 * t5344 * t26403 * t6415 - 6.0 * t5234 * t28178 + 0.49348022005446793095e-1 * t91081 + 0.24674011002723396548e-1 * t97494 - 0.19739208802178717238e0 * t107431 - 0.49348022005446793095e-1 * t107435 + 0.49348022005446793095e-1 * t107439 + t20595 * t2013;
    (t107442,)
}
