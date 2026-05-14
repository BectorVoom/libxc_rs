//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 606/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk606<F: Float>(t2646: F, t2652: F, t2657: F, t2660: F, t2662: F, t2665: F, t2670: F, t2672: F, t2678: F, t2682: F, t2685: F, t336: F, t363: F, t925: F, t931: F, t951: F, t967: F) -> (F,) {
    let t2688 = -t925 * t2646 / 144.0 - t2652 + 19.0 / 1728.0 * t2657 * t363 - t2660 / 432.0 + 11.0 / 108.0 * t2662 * t336 - t2665 / 54.0 - t2670 - t967 * t2672 / 2304.0 + t2678 / 2304.0 - t2682 * t951 / 288.0 - t2685 * t931 / 54.0;
    (t2688,)
}
