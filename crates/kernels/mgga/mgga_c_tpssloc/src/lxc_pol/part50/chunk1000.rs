//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1000/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1000<F: Float>(t1539: F, t30781: F, t23329: F, t1599: F, t30861: F, t25406: F, t8380: F, t23394: F, t7599: F, t6704: F, t1052: F, t1920: F, t1956: F, t23327: F, t25755: F, t25778: F, t30798: F, t32909: F, t32913: F, t32917: F, t32924: F, t32965: F, t4557: F, t6687: F, t8407: F) -> (F, F, F, F, F, F, F) {
    let t32969 = t30781 * t1539;
    let t32970 = t23329 * t32969;
    let t32973 = t1599 * t30861;
    let t32976 = t25406 * t8380;
    let t32980 = t23394 * t7599;
    let t32981 = t6704 * t32980;
    let t32984 = -6.0 * t1052 * t32909 + 2.0 * t1052 * t32913 + 4.0 * t1052 * t32917 - 2.0 * t25755 * t1956 + 0.16449340668482264365e-1 * t1920 * t32924 - t1052 * t32965 - 2.0 * t25778 * t1956 - 0.54831135561607547883e-2 * t23327 * t32970 + 0.16449340668482264365e-1 * t6687 * t32973 - 0.16449340668482264365e-1 * t6687 * t32976 - t4557 * t8407 + t30798 + 0.3289868133696452873e-1 * t6687 * t32981;
    (t32969, t32970, t32973, t32976, t32980, t32981, t32984)
}
