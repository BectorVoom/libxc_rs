//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 906/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk906<F: Float>(t5348: F, t6420: F, t20473: F, t5335: F, t1380: F, t20554: F, t6415: F, t1825: F, t19657: F, t16428: F, t6388: F, t1336: F, t1814: F, t1838: F, t1840: F, t19815: F, t20595: F, t20616: F, t20622: F, t20625: F, t20630: F, t20632: F, t5234: F, t5334: F, t5344: F, t544: F, t564: F, t6378: F, t6448: F, t6451: F, t6454: F, t6456: F, t6458: F) -> (F, F, F, F, F, F, F) {
    let t20635 = t5348 * t6420;
    let t20638 = t5335 * t20473;
    let t20643 = t1380 * t20554;
    let t20645 = t5348 * t6415;
    let t20648 = t19657 * t1825;
    let t20651 = t16428 * t6388;
    let t20661 = -6.0 * t1336 * t20622 + 6.0 * t1336 * t20625 - t1336 * t20630 - 3.0 * t1336 * t20635 - t1336 * t20643 - 3.0 * t1336 * t20645 - 3.0 * t1336 * t20648 + 6.0 * t1336 * t20651 + 3.0 * t1814 * t6458 - 3.0 * t1838 * t19815 + 3.0 * t1840 * t6378 + t20595 * t564 + t20616 * t544 - 3.0 * t20632 * t5344 + 6.0 * t20638 * t5334 + 6.0 * t5234 * t6448 - 6.0 * t5234 * t6451 - 3.0 * t5234 * t6454 - 3.0 * t5234 * t6456;
    (t20635, t20638, t20643, t20645, t20648, t20651, t20661)
}
