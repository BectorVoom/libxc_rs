//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1197/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1197<F: Float>(t1992: F, t26404: F, t6976: F, t22897: F, t26453: F, t114097: F, t114105: F, t1985: F, t1998: F, t214: F, t26328: F, t32749: F, t6883: F, t32748: F, t6897: F, t794: F) -> (F, F, F, F, F, F, F) {
    let t120502 = 0.16449340668482264365e-1 * t1992 * t6976 * t26404;
    let t120505 = 0.3289868133696452873e-1 * t1992 * t22897 * t26453;
    let t120506 = 0.82246703342411321825e-2 * t114097;
    let t120507 = 0.38381794893125283518e-1 * t114105;
    let t120513 = 0.16449340668482264365e-1 * t1985 * t214 * t1998 * t26328;
    let t120514 = t6883 * t32749;
    let t120515 = 0.38381794893125283518e-1 * t120514;
    let t120521 = t6897 * t794 * t32748;
    (t120502, t120505, t120506, t120507, t120513, t120515, t120521)
}
