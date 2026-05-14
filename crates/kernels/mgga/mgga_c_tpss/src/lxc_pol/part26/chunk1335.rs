//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1335/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1335<F: Float>(t13458: F, t13463: F, t13478: F, t13565: F, t1897: F, t19261: F, t20706: F, t20950: F, t22108: F, t3493: F, t4638: F, t4641: F, t5986: F, t5991: F, t6054: F, t626: F, t645: F, t68891: F, t68905: F, t68907: F, t68909: F, t68913: F, t68915: F, t68917: F, t68919: F, t68921: F, t68923: F) -> (F,) {
    let t72819 = -2.0 * t22108 * t626 * t645 - 2.0 * t13458 * t1897 - 2.0 * t13463 * t5986 - 4.0 * t13478 * t5986 - 2.0 * t13565 * t5991 - 4.0 * t19261 * t4641 - 4.0 * t20706 * t4641 - 4.0 * t20950 * t3493 - 2.0 * t4638 * t6054 - t68891 + t68905 + t68907 - t68909 - t68913 - t68915 - t68917 - t68919 - t68921 - t68923;
    (t72819,)
}
