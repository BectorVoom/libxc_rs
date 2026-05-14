//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1038/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1038<F: Float>(t22986: F, t22996: F, t25249: F, t5585: F, t1880: F, t1894: F, t21013: F, t214: F, t1888: F, t232: F, t6646: F, t67358: F, t1484: F, t6552: F, t6637: F, t98598: F) -> (F, F, F, F) {
    let t105551 = t22986 * t22996 * t25249 * t5585;
    let t105561 = t1880 * t214 * t1894 * t21013;
    let t105565 = t1888 * t6646 * t67358 * t232;
    let t105574 = t6552 * t6637 * t98598 * t1484;
    (t105551, t105561, t105565, t105574)
}
