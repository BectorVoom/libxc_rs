//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 786/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk786<F: Float>(t9287: F, t9288: F, t2267: F, t607: F, t2250: F, t43: F, t9258: F, t53: F, t54: F, t2274: F, t55: F, t2585: F, t2262: F, t2268: F, t2271: F, t39: F, t44: F, t51: F, t615: F, t618: F, t9277: F) -> (F,) {
    let t9289 = t9287 * t9288;
    let t9292 = t2267 * t607;
    let t9293 = t9292 * t2250;
    let t9296 = t43 * t9258;
    let t9300 = 1.0 / t54 / t53;
    let t9301 = t9300 * t9288;
    let t9304 = t2274 * t607;
    let t9305 = t9304 * t2250;
    let t9308 = t55 * t9258;
    let t9311 = 1232.0 / 27.0 * t2585;
    let t9312 = -1232.0 / 27.0 * t9277 * t44 + 220.0 / 9.0 * t2262 * t618 - 20.0 / 9.0 * t615 * t2268 - 20.0 / 3.0 * t615 * t2271 - 5.0 / 108.0 * t39 * t9289 + 5.0 / 6.0 * t39 * t9293 + 5.0 / 6.0 * t39 * t9296 + 5.0 / 108.0 * t51 * t9301 + 5.0 / 6.0 * t51 * t9305 - 5.0 / 6.0 * t51 * t9308 + t9311;
    (t9312,)
}
