//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1102/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1102<F: Float>(t32927: F, t6784: F, t1599: F, t8400: F, t6800: F, t7619: F, t6799: F, t1948: F, t7593: F, t345: F, t1615: F, t8391: F) -> (F, F, F, F, F, F, F) {
    let t32928 = t6784 * t32927;
    let t32931 = t1599 * t8400;
    let t32934 = t7619 * t6800;
    let t32935 = t6799 * t32934;
    let t32938 = t1948 * t7593;
    let t32939 = t345 * t32938;
    let t32943 = t8391 * t1615;
    (t32928, t32931, t32934, t32935, t32938, t32939, t32943)
}
