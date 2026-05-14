//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 985/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk985<F: Float>(t31137: F, t7700: F, t1985: F, t1799: F, t31193: F, t6637: F, t6888: F, t26403: F, t550: F, t6976: F, t1992: F, t1998: F, t7722: F, t214: F, t1825: F, t31211: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32735 = t31137 * t7700;
    let t32737 = 0.16449340668482264365e-1 * t1985 * t32735;
    let t32740 = t31193 * t1799;
    let t32741 = t6637 * t32740;
    let t32743 = 0.3289868133696452873e-1 * t6888 * t32741;
    let t32744 = t26403 * t550;
    let t32745 = t6976 * t32744;
    let t32747 = 0.16449340668482264365e-1 * t1992 * t32745;
    let t32748 = t1998 * t7722;
    let t32749 = t214 * t32748;
    let t32751 = 0.16449340668482264365e-1 * t1985 * t32749;
    let t32753 = t31211 * t1825;
    (t32735, t32737, t32740, t32741, t32743, t32744, t32745, t32747, t32748, t32749, t32751, t32753)
}
