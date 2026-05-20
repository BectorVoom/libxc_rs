//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3163/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3163<F: Float>(t1213: F, t18941: F, t248: F, t3570: F, t15730: F, t5019: F, t1216: F, t3966: F, t1227: F, t1230: F, t15495: F, t15498: F, t15708: F, t15710: F, t15740: F, t1737: F, t1748: F, t19051: F, t3527: F, t3531: F, t3577: F, t3578: F, t3585: F, t44929: F, t44932: F, t4728: F, t5014: F, t5030: F, t53406: F, t53507: F, t5971: F, t6227: F, t6232: F, t63357: F, t63363: F) -> (F, F) {
    let t65424 = t1213 * t248 * t3570 * t18941;
    let t65444 = t5019 * t15730;
    let t65452 = t1216 * t3966;
    let t65463 = t65424 / F::new(2304.0) - t1227 * t248 * t1230 * t63363 / F::new(1152.0) + t53507 * t1748 / F::new(432.0) + t15498 * t5030 / F::new(216.0) + t44929 * t6227 / F::new(1536.0) - t19051 * t3527 / F::new(4608.0) - t19051 * t3531 / F::new(2304.0) - t53406 * t1737 / F::new(288.0) - t15495 * t5014 / F::new(144.0) + t65444 / F::new(1296.0) + F::new(5.0) / F::new(6912.0) * t1227 * t248 * t3585 * t63357 - t44932 * t6232 / F::new(3072.0) - t3577 * t3578 * t4728 * t65452 / F::new(576.0) - t3577 * t3578 * t5971 * t15708 / F::new(384.0) - t15740 * t15710 / F::new(576.0);
    (t65452, t65463)
}
