//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1258/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1258<F: Float>(t1036: F, t23557: F, t10469: F, t3127: F, t363: F, t1933: F, t1937: F, t2250: F, t3200: F, t83015: F, t1030: F, t1058: F, t3068: F, t1004: F, t1015: F, t10410: F, t10415: F, t10857: F, t23419: F, t23457: F, t23483: F, t23495: F, t23504: F, t23515: F, t23521: F, t23548: F, t23556: F, t23564: F, t25652: F, t25654: F, t25660: F, t3073: F, t3120: F, t3128: F, t3131: F, t360: F, t378: F, t6723: F, t6730: F, t6735: F, t6742: F, t6744: F, t68: F, t82911: F, t82987: F, t82990: F, t83117: F, sigma0: F) -> (F,) {
    let t83172 = t23557 * t1036;
    let t83196 = t10469 * t3127 * t363;
    let t83206 = t1933 * t2250 * t1937;
    let t83215 = t3200 * t83015;
    let t83220 = t1058 * sigma0 * t1030 * t3068;
    let t83223 = 19.0 / 432.0 * t83172 + 19.0 / 288.0 * t1004 * t23556 * t378 - 0.30279567070605293142e-3 * t23564 * t23504 + 0.60559134141210586284e-3 * t25652 * t3128 * t3120 * t25654 - 0.30279567070605293142e-3 * t25652 * t1015 * t3120 * t25660 - 0.60559134141210586284e-3 * t82911 * t23515 + 0.48447307312968469026e-2 * t23457 * t6735 - 0.30279567070605293142e-3 * t6730 * t23548 + 0.24223653656484234513e-2 * t6723 * t23495 - 0.60559134141210586284e-3 * t82987 * t83196 * t82990 * t3131 - 0.30279567070605293142e-3 * t83117 * t23521 - 0.24223653656484234513e-2 * t23483 * t23504 + 0.30279567070605293142e-3 * t83206 + 0.10093189023535097714e-3 * t6742 * t6744 * t10857 * t68 * t360 + 5.0 / 2304.0 * t23419 * t10410 - t83215 * t10415 / 768.0 - t83220 * t3073 / 72.0;
    (t83223,)
}
