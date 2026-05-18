//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1302/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1302<F: Float>(t1206: F, t5458: F, t6242: F, t7309: F, t14001: F, t196: F, t197: F, t1268: F, t21011: F, t1338: F, t3490: F, t1321: F, t3537: F) -> (F, F, F, F, F, F) {
    let t68958 = t5458 * t1206;
    let t68967 = t6242 * t7309;
    let t68975 = t14001 * t196 * t197;
    let t68989 = t21011 * t1268;
    let t69023 = t3490 * t1338;
    let t69026 = t1321 * t3537;
    (t68958, t68967, t68975, t68989, t69023, t69026)
}
