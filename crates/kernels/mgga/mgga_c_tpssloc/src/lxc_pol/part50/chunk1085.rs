//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1085/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1085<F: Float>(t32970: F, t82431: F, t23384: F, t32981: F, t32998: F, t113243: F, t1927: F, t1946: F, t1956: F, t23327: F, t23365: F, t23372: F, t254: F, t25406: F, t25424: F, t25429: F, t25431: F, t25732: F, t25759: F, t25778: F, t25815: F, t3026: F, t30805: F, t30861: F, t30868: F, t30904: F, t32909: F, t32980: F, t4548: F, t4557: F, t4660: F, t6687: F, t6771: F, t6776: F, t7600: F, t89666: F) -> (F,) {
    let t119444 = t82431 * t32970;
    let t119446 = t23384 * t32981;
    let t119467 = t23384 * t32998;
    let t119485 = 0.16449340668482264365e-1 * t1927 * t4548 * t30861 - 0.18277045187202515961e-2 * t119444 + 0.10966227112321509577e-1 * t119446 - 2.0 * t89666 * t1956 + 0.3289868133696452873e-1 * t6687 * t23365 * t32980 - 6.0 * t4557 * t30805 + 4.0 * t23372 * t7600 - 6.0 * t3026 * t32909 - 12.0 * t1946 * t254 * t25759 + 0.3289868133696452873e-1 * t6687 * t25406 * t30904 - 6.0 * t4660 * t30805 - 0.54831135561607547883e-2 * t119467 - 0.16449340668482264365e-1 * t6687 * t25406 * t30868 - 0.54831135561607547883e-2 * t23327 * t113243 * t25815 - 0.10966227112321509577e-1 * t23327 * t113243 * t25424 + 0.73108180748810063844e-2 * t25429 * t113243 * t25431 - 2.0 * t6771 * t25732 + 4.0 * t25778 * t6776;
    (t119485,)
}
