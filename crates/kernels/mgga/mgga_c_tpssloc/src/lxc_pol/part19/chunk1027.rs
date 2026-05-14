//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1027/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1027<F: Float>(t11152: F, t76: F, t2244: F, t2250: F, t2251: F, t2252: F, t2255: F, t2283: F, t2284: F, t2291: F, t2298: F, t2304: F, t39096: F, t39097: F, t39103: F, t39110: F, t608: F, t609: F, t629: F, t634: F, t638: F, t642: F, t66: F, t72: F, t80: F, t9258: F, t9263: F, t9268: F, t9312: F, t9313: F, t9321: F, t9324: F, t9330: F, t9333: F, t9339: F) -> (F,) {
    let t39114 = 1.0 / t76 / t11152;
    let t39130 = -t2251 * t2283 * t80 / 2.0 - t9263 * t642 - t2252 * t2304 / 2.0 - t608 * t9312 * t80 / 3.0 - t9268 * t642 - t2255 * t2304 - t609 * t9339 / 3.0 + t9313 * t642 / 6.0 + t2284 * t2304 / 4.0 + t629 * t9339 / 6.0 + t66 * t72 * (3640.0 / 81.0 * t39096 * t39097 - 560.0 / 9.0 * t9321 * t2244 * t2250 + 28.0 / 3.0 * t2291 * t39103 + 112.0 / 9.0 * t9324 * t9258 - 4.0 / 3.0 * t634 * t39110 + 3640.0 / 81.0 * t39114 * t39097 + 560.0 / 9.0 * t9330 * t2244 * t2250 + 28.0 / 3.0 * t2298 * t39103 + 112.0 / 9.0 * t9333 * t9258 + 4.0 / 3.0 * t638 * t39110) / 24.0;
    (t39130,)
}
