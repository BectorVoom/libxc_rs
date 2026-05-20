//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1777/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1777<F: Float>(t23171: F, t23228: F, t6572: F, t212: F, t6554: F, t852: F, t23030: F, t23253: F, t6555: F, t81573: F, t6563: F, t81597: F) -> (F, F, F, F, F) {
    let t82082 = t23171 * t23228 * t6572;
    let t82087 = t23171 * t212 * t852 * t6554;
    let t82099 = t23030 * t23253;
    let t82120 = t81573 * t23228 * t6555;
    let t82122 = t81597 * t6563;
    (t82082, t82087, t82099, t82120, t82122)
}
