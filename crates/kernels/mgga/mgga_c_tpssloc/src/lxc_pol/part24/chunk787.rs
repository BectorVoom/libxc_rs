//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 787/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk787<F: Float>(t33: F, t9312: F, t2769: F, t73: F, t2291: F, t607: F, t3241: F, t76: F, t2298: F, t2250: F, t634: F, t638: F, t9258: F, t9288: F, t72: F, t2245: F, t2252: F, t2255: F, t2284: F, t2304: F, t609: F, t629: F, t642: F, t66: F, t80: F, t9247: F, t9248: F, t9251: F, t9260: F, t9263: F, t9268: F) -> (F, F) {
    let t9313 = t33 * t9312;
    let t9321 = 1.0 / t73 / t2769;
    let t9324 = t2291 * t607;
    let t9330 = 1.0 / t76 / t3241;
    let t9333 = t2298 * t607;
    let t9338 = -280.0 / 27.0 * t9321 * t9288 + 28.0 / 3.0 * t9324 * t2250 - 4.0 / 3.0 * t634 * t9258 + 280.0 / 27.0 * t9330 * t9288 + 28.0 / 3.0 * t9333 * t2250 + 4.0 / 3.0 * t638 * t9258;
    let t9339 = t72 * t9338;
    let t9342 = -t9247 * t9248 / 4.0 - t9251 * t80 / 4.0 - t2245 * t642 / 4.0 - t9260 * t80 / 12.0 - t9263 * t80 / 4.0 - t2252 * t642 / 4.0 - t9268 * t80 / 4.0 - t2255 * t642 / 2.0 - t609 * t2304 / 4.0 + t9313 * t80 / 24.0 + t2284 * t642 / 8.0 + t629 * t2304 / 8.0 + t66 * t9339 / 24.0;
    (t9338, t9342)
}
