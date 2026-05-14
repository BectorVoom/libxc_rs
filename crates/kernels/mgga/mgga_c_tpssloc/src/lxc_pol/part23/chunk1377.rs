//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1377/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1377<F: Float>(t12249: F, t1336: F, t1375: F, t1378: F, t1380: F, t16047: F, t16428: F, t1814: F, t1825: F, t1834: F, t1840: F, t1842: F, t1843: F, t19657: F, t19743: F, t19815: F, t20029: F, t20060: F, t20473: F, t20495: F, t20594: F, t20595: F, t20613: F, t20616: F, t20625: F, t20635: F, t20648: F, t20651: F, t20661: F, t3887: F, t3897: F, t40541: F, t5215: F, t5234: F, t5321: F, t5334: F, t562: F, t564: F, t568: F, t57653: F, t6361: F, t6378: F, t6388: F, t6415: F, t6434: F, t6440: F, t6448: F, t6458: F, t6461: F, t74849: F, t74930: F, t75008: F, t75124: F, t79993: F, t80048: F, t80076: F, t80164: F, t80175: F, t80181: F, t80185: F, t80189: F, t80193: F, t80482: F) -> (F,) {
    let t80489 = -4.0 * t74849 * t1843 - 4.0 * t74930 * t1843 + 6.0 * t1375 * t3887 * t79993 - 12.0 * t20029 * t6461 + 8.0 * t1375 * t3887 * t1842 * t20661 + 24.0 * t5321 * t20613 + 24.0 * t5215 * t20613 + 24.0 * t20029 * t6440 + t80048 * t562 * t568 + 6.0 * t6361 * t6434 * t568 + 4.0 * t20594 * t1834 * t568 - t1375 * t1378 * (-3.0 * t1336 * t1380 * t80076 + 24.0 * t1336 * t16428 * t20495 - 4.0 * t1336 * t75124 * t1825 - 6.0 * t1336 * t19657 * t6415 + 12.0 * t1336 * t57653 * t6388 + 12.0 * t19815 * t6448 + 24.0 * t5234 * t20625 - 12.0 * t5234 * t20635 - 12.0 * t5234 * t20648 + 24.0 * t5234 * t20651 + t80164 - 36.0 * t1336 * t12249 * t80189 - t1336 * t1380 * t80193 + 14.0 * t1336 * t3897 * t80181 + 24.0 * t1336 * t40541 * t80185 - 36.0 * t16047 * t19743 * t75008 + 36.0 * t5334 * t19743 * t20473 + 4.0 * t1814 * t20616 + 4.0 * t20595 * t1840 + t80175 * t564 + 6.0 * t6378 * t6458 + t80482) - 6.0 * t20060 * t6461;
    (t80489,)
}
