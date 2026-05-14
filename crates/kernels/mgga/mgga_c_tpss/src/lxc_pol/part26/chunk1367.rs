//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1367/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1367<F: Float>(t1673: F, t6547: F, t22217: F, t546: F, t1901: F, t5480: F, t1284: F, t16041: F, t16079: F, t1902: F, t1906: F, t22199: F, t3: F, t4544: F, t4562: F, t550: F, t6062: F, t6548: F, t6556: F, t68782: F, t68786: F, t68788: F, t73604: F) -> (F,) {
    let t73638 = t6547 * t1673;
    let t73640 = t546 * t22217;
    let t73641 = t1901 * t5480;
    let t73642 = t3 * t550 * t73604 + t1284 * t22199 + t16041 * t1906 + t16079 * t1902 + 2.0 * t4544 * t6556 + 2.0 * t4562 * t6548 + t5480 * t6062 + t68782 + t68786 + t68788 + 2.0 * t73638 + t73640 + t73641;
    (t73642,)
}
