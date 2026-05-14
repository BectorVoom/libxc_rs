//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1037/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1037<F: Float>(t6883: F, t8455: F, t8459: F, t22666: F, t8458: F, t1985: F, t6906: F, t6992: F, t6889: F, t22674: F, t6897: F, t2006: F, t214: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31113 = 0.38381794893125283518e-1 * t6883 * t8455;
    let t31115 = 0.38381794893125283518e-1 * t6883 * t8459;
    let t31120 = t22666 * t8458;
    let t31122 = 0.16449340668482264365e-1 * t1985 * t31120;
    let t31123 = t6906 * t6992;
    let t31124 = t6889 * t31123;
    let t31126 = 0.16449340668482264365e-1 * t1985 * t31124;
    let t31127 = t22674 * t8458;
    let t31129 = 0.82246703342411321825e-2 * t6897 * t31127;
    let t31137 = t214 * t2006;
    (t31113, t31115, t31120, t31122, t31123, t31124, t31126, t31127, t31129, t31137)
}
