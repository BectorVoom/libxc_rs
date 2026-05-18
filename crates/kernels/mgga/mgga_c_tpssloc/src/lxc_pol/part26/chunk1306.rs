//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1306/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1306<F: Float>(t23228: F, t6555: F, t81573: F, t6563: F, t81597: F, t214: F, t2710: F, t1880: F, t6572: F, t23196: F, t23237: F, t23258: F, t6547: F) -> (F, F, F, F, F, F) {
    let t82120 = t81573 * t23228 * t6555;
    let t82122 = t81597 * t6563;
    let t82123 = F::new(0.16220877603642232915e0) * t82122;
    let t82124 = t214 * t2710;
    let t82126 = t1880 * t82124 * t6572;
    let t82129 = t1880 * t23237 * t23196;
    let t82131 = t6547 * t23258;
    (t82120, t82123, t82124, t82126, t82129, t82131)
}
