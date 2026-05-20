//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 905/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk905<F: Float>(t18371: F, t3577: F, t248: F, t3570: F, t6219: F, t1213: F, t3521: F, t5975: F, t1227: F, t1409: F, t15701: F, t3450: F, t5398: F) -> (F, F, F, F, F, F, F) {
    let t18372 = t3577 * t18371;
    let t18375 = t248 * t3570 * t6219;
    let t18376 = t1213 * t18375;
    let t18392 = t248 * t3521 * t5975;
    let t18393 = t1227 * t18392;
    let t18395 = t15701 * t1409;
    let t18409 = t3450 * t5398;
    (t18372, t18375, t18376, t18392, t18393, t18395, t18409)
}
