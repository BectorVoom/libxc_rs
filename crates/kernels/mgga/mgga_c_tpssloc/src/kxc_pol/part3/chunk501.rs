//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 501/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk501<F: Float>(t2283: F, t33: F, t40: F, t632: F, t73: F, t52: F, t636: F, t76: F, t2244: F, t2250: F, t634: F, t638: F, t72: F, t2245: F, t2252: F, t2255: F, t609: F, t629: F, t642: F, t66: F, t80: F) -> (F, F, F, F, F, F, F) {
    let t2284 = t33 * t2283;
    let t2289 = t632 * t40;
    let t2291 = 1.0 / t73 / t2289;
    let t2296 = t636 * t52;
    let t2298 = 1.0 / t76 / t2296;
    let t2303 = 28.0 / 9.0 * t2291 * t2244 - 4.0 / 3.0 * t634 * t2250 + 28.0 / 9.0 * t2298 * t2244 + 4.0 / 3.0 * t638 * t2250;
    let t2304 = t72 * t2303;
    let t2307 = -t2245 * t80 / 12.0 - t2252 * t80 / 12.0 - t2255 * t80 / 6.0 - t609 * t642 / 6.0 + t2284 * t80 / 24.0 + t629 * t642 / 12.0 + t66 * t2304 / 24.0;
    (t2284, t2289, t2291, t2296, t2298, t2304, t2307)
}
