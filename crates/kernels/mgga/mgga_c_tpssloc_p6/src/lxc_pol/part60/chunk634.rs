//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 634/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk634<F: Float>(t1734: F, t68: F, t475: F, t7328: F, t1730: F, t2140: F, t1742: F, t2139: F, t471: F, t1726: F, t1737: F, t1748: F, t2134: F, t2136: F, t467: F, t488: F, t7309: F, t7310: F, t7315: F, t7326: F, t7339: F, t7343: F, t7345: F, t8020: F, t8028: F, t8031: F, t8035: F) -> (F, F, F, F, F, F) {
    let t8038 = t1734 * t68;
    let t8039 = t8038 * t475;
    let t8040 = t7328 * t8039;
    let t8043 = t1730 * t2140;
    let t8048 = t2139 * t1742;
    let t8049 = t471 * t8048;
    let t8054 = -t8020 * t467 / F::new(36.0) + t7309 - t7310 * t1726 / F::new(288.0) - F::cast_from(0.80745512188280781712e-3_f64) * t8028 * t2136 + t7315 - F::cast_from(0.10093189023535097714e-3_f64) * t8031 * t2136 - F::cast_from(0.10093189023535097714e-3_f64) * t2134 * t8035 + F::cast_from(0.10093189023535097714e-3_f64) * t7326 * t8040 + t8043 * t488 / F::new(1536.0) + t7339 * t1737 / F::new(1536.0) - t8049 * t488 / F::new(288.0) + t7343 - t7345 * t1748 / F::new(2304.0);
    (t8039, t8040, t8043, t8048, t8049, t8054)
}
