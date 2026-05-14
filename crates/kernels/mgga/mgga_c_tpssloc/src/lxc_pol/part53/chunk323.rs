//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 323/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk323<F: Float>(t1174: F, t1195: F, t1213: F, t1224: F, t1227: F, t1706: F, t1726: F, t1731: F, t1737: F, t1744: F, t1748: F, t467: F, t488: F, t466: F, t1734: F, t491: F) -> (F, F, F) {
    let t1751 = -t1706 * t467 / 36.0 + t1195 - t1174 * t1726 / 288.0 + t1731 * t488 / 3072.0 + t1213 * t1737 / 3072.0 - t1744 * t488 / 576.0 + t1224 - t1227 * t1748 / 4608.0;
    let t1752 = t466 * t1751;
    let t1755 = t491 * t1734;
    (t1751, t1752, t1755)
}
