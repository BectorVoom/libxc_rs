//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1182/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1182<F: Float>(t103: F, t110333: F, t110334: F, t110336: F, t110338: F, t110340: F, t111101: F, t111104: F, t111109: F, t111111: F, t111121: F, t111125: F, t111127: F, t111129: F, t111134: F, t12808: F, t2195: F, t2332: F, t2350: F, t2354: F, t2358: F, t2585: F, t29903: F, t30056: F, t30293: F, t30297: F, t35656: F, t35663: F, t4059: F, t8128: F, t8137: F, t8180: F) -> (F,) {
    let t111141 = 44.0 / 9.0 * t110334 - 110.0 / 27.0 * t110336 - 2.0 / 3.0 * t110338 + 5.0 / 9.0 * t110340 + t110333 + 22.0 / 9.0 * t111101 - t111104 + t8128 * t8180 * t12808 / 4.0 + t111109 - t111111 - 5.0 / 12.0 * t8128 * t30293 * t2358 + 25.0 / 72.0 * t8137 * t30297 * t2354 + 5.0 / 4.0 * t29903 * t30293 * t2332 + 25.0 / 108.0 * t8137 * t111121 * t2350 - 55.0 / 27.0 * t111125 - 125.0 / 72.0 * t111127 + 55.0 / 27.0 * t111129 + 5.0 / 24.0 * t2585 * t2195 * t103 - 5.0 / 2.0 * t35656 * t111134 * t30056 + 5.0 / 9.0 * t35663 * t4059 * t30056;
    (t111141,)
}
