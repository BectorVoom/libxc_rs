//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 879/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk879<F: Float>(t120514: F, t120521: F, t1992: F, t550: F, t6976: F, t97172: F, t22897: F, t3792: F, t120605: F, t120610: F, t120197: F, t1799: F, t22633: F, t22635: F, t1842: F, t31090: F) -> (F, F, F, F, F, F, F, F, F) {
    let t127403 = 0.76763589786250567036e-1 * t120514;
    let t127404 = 0.16449340668482264365e-1 * t120521;
    let t127408 = 0.16449340668482264365e-1 * t1992 * t6976 * t97172 * t550;
    let t127412 = 0.3289868133696452873e-1 * t1992 * t22897 * t97172 * t3792;
    let t127422 = 0.15352717957250113407e0 * t120605;
    let t127423 = 0.76763589786250567036e-1 * t120610;
    let t127427 = 0.6579736267392905746e-1 * t22633 * t22635 * t120197 * t1799;
    let t127430 = t1799 * t1842;
    let t127434 = 0.13159472534785811492e0 * t22633 * t22635 * t31090 * t127430;
    (t127403, t127404, t127408, t127412, t127422, t127423, t127427, t127430, t127434)
}
