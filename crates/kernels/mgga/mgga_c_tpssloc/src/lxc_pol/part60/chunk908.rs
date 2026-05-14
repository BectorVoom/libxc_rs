//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 908/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk908<F: Float>(t28860: F, t8607: F, t19596: F, t1983: F, t8640: F, t1458: F, t33553: F, t652: F, t1873: F, t29197: F, t2018: F, t24432: F, t24995: F, t6330: F, t26161: F, t6324: F, t92169: F) -> (F, F, F, F, F, F) {
    let t128475 = t8607 * t28860;
    let t128477 = t1983 * t8640 * t19596;
    let t128482 = 4.0 * t652 * t33553 * t1458;
    let t128485 = 2.0 * t652 * t29197 * t1873;
    let t128492 = 6.0 * t24995 * t24432 * t2018 * t6330;
    let t128498 = 6.0 * t26161 * t92169 * t2018 * t6324;
    (t128475, t128477, t128482, t128485, t128492, t128498)
}
