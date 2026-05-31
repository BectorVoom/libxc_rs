//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2486/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2486<F: Float>(t25548: F, t360: F, t10403: F, t10408: F, t13995: F, t17177: F, t17182: F, t17920: F, t17925: F, t17972: F, t3070: F, t3071: F, t3130: F, t4582: F, t4594: F, t4644: F, t49934: F, t5681: F, t62494: F, t62499: F, t62510: F, t62515: F, t70082: F, t70391: F) -> F {
    let t70735 = t25548 * t360;
    let t70756 = -t62494 / F::cast_from(3456.0_f64) - t62499 / F::cast_from(216.0_f64) - t10403 * t3071 * t5681 * t70082 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t3070 * t10408 * t17177 * t70735 + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t13995 * t17920 - t49934 * t17925 / F::cast_from(768.0_f64) - t3070 * t3071 * t17182 * t70735 / F::cast_from(768.0_f64) + t3130 * t4582 * t70391 * t4594 / F::cast_from(1536.0_f64) + t4644 * t17972 / F::cast_from(256.0_f64) - t62510 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t62515;
    t70756
}
