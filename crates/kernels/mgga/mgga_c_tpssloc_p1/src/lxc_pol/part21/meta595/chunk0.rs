//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2345/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2345<F: Float>(t1799: F, t3792: F, t6414: F, t1484: F, t2632: F, t5611: F, t154: F, t2558: F, t10: F, t2229: F, t116: F, t117: F) -> (F, F, F, F, F, F, F) {
    let t20468 = t3792 * t1799;
    let t20473 = t3792 * t6414;
    let t20981 = t2632 * t1484;
    let t20986 = t2632 * t5611;
    let t22715 = t2558 * t154;
    let t22811 = t2229 * t10;
    let t22815 = t117 * t116;
    (t20468, t20473, t20981, t20986, t22715, t22811, t22815)
}
