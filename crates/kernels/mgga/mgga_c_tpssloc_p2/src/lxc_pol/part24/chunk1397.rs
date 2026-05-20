//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1397/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1397<F: Float>(t1004: F, t1015: F, t10410: F, t10415: F, t10857: F, t23419: F, t23457: F, t23483: F, t23495: F, t23504: F, t23515: F, t23521: F, t23548: F, t23556: F, t23564: F, t25652: F, t25654: F, t25660: F, t3073: F, t3120: F, t3128: F, t3131: F, t360: F, t378: F, t6723: F, t6730: F, t6735: F, t6742: F, t6744: F, t68: F, t82911: F, t82987: F, t82990: F, t83117: F, t83172: F, t83196: F, t83206: F, t83215: F, t83220: F) -> F {
    let t83223 = F::new(19.0) / F::new(432.0) * t83172 + F::new(19.0) / F::new(288.0) * t1004 * t23556 * t378 - F::cast_from(0.30279567070605293142e-3_f64) * t23564 * t23504 + F::cast_from(0.60559134141210586284e-3_f64) * t25652 * t3128 * t3120 * t25654 - F::cast_from(0.30279567070605293142e-3_f64) * t25652 * t1015 * t3120 * t25660 - F::cast_from(0.60559134141210586284e-3_f64) * t82911 * t23515 + F::cast_from(0.48447307312968469026e-2_f64) * t23457 * t6735 - F::cast_from(0.30279567070605293142e-3_f64) * t6730 * t23548 + F::cast_from(0.24223653656484234513e-2_f64) * t6723 * t23495 - F::cast_from(0.60559134141210586284e-3_f64) * t82987 * t83196 * t82990 * t3131 - F::cast_from(0.30279567070605293142e-3_f64) * t83117 * t23521 - F::cast_from(0.24223653656484234513e-2_f64) * t23483 * t23504 + F::cast_from(0.30279567070605293142e-3_f64) * t83206 + F::cast_from(0.10093189023535097714e-3_f64) * t6742 * t6744 * t10857 * t68 * t360 + F::new(5.0) / F::new(2304.0) * t23419 * t10410 - t83215 * t10415 / F::new(768.0) - t83220 * t3073 / F::new(72.0);
    t83223
}
