//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1446/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1446<F: Float>(t104018: F, t104085: F, t104088: F, t104094: F, t104303: F, t104387: F, t104413: F, t1737: F, t1748: F, t22197: F, t22243: F, t22246: F, t22275: F, t24733: F, t27684: F, t27711: F, t29594: F, t29644: F, t29648: F, t475: F, t68: F, t7326: F, t7328: F, t7339: F, t7345: F, t8040: F, t95327: F, t95335: F) -> F {
    let t109493 = t7339 * t22246 / F::cast_from(1536.0_f64) - t104018 * t1748 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t7345 * t22197 - t24733 * t22275 / F::cast_from(512.0_f64) - t104085 / F::cast_from(288.0_f64) - t104088 / F::cast_from(144.0_f64) - t104094 / F::cast_from(576.0_f64) - F::cast_from(0.24223653656484234513e-2_f64) * t27711 * t29594 - F::cast_from(0.30279567070605293142e-3_f64) * t27684 * t29594 + F::cast_from(0.10093189023535097714e-3_f64) * t7326 * t7328 * t22243 * t68 * t475 - F::cast_from(0.30279567070605293142e-3_f64) * t104387 * t8040 - t95335 / F::cast_from(2304.0_f64) + F::cast_from(0.48447307312968469026e-2_f64) * t104413 * t8040 - F::cast_from(0.48447307312968469026e-2_f64) * t95327 * t29644 + F::cast_from(0.24223653656484234513e-2_f64) * t95327 * t29648 - t104303 * t1737 / F::cast_from(48.0_f64);
    t109493
}
