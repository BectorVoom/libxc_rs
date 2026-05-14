//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1280/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1280<F: Float>(t1824: F, t8617: F, t1338: F, t33266: F, t1985: F, t1998: F, t214: F, t27051: F, t1992: F, t550: F, t6976: F, t93505: F, t114064: F, t115397: F, t115409: F, t115415: F, t115423: F, t115486: F, t120468: F, t120469: F, t120471: F, t1336: F, t1352: F, t1825: F, t31637: F, t33289: F, t3777: F, t5230: F, t5234: F, t5250: F, t5334: F, t8634: F) -> (F, F) {
    let t122471 = t8617 * t1824;
    let t122475 = t1338 * t33266;
    let t122483 = t1985 * t214 * t1998 * t27051;
    let t122488 = t1992 * t6976 * t93505 * t550;
    let t122495 = 2.0 * t5334 * t122471 * t5250 - t1336 * t122475 * t1352 + 0.38381794893125283518e-1 * t115397 + 0.82246703342411321824e-2 * t115409 + t5230 * t8634 + 0.82246703342411321825e-2 * t122483 + 0.19190897446562641759e-1 * t115415 + t120468 + t120469 + t120471 - t114064 - 0.82246703342411321825e-2 * t122488 + 0.41123351671205660912e-2 * t115423 - t5234 * t31637 - t3777 * t33289 - t1336 * t115486 * t1825;
    (t122471, t122495)
}
