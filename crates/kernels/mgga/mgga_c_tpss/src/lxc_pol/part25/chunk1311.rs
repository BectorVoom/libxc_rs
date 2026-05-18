//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1311/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1311<F: Float>(t13800: F, t5728: F, t13858: F, t13856: F, t19469: F, t215: F, t13731: F, t5716: F, t13677: F, t18454: F, t13853: F, t5721: F) -> (F, F, F, F, F, F) {
    let t69539 = t5728 * t13800;
    let t69541 = t5728 * t13858;
    let t69544 = t19469 * t215 * t13856;
    let t69546 = t5716 * t13731;
    let t69548 = t18454 * t13677;
    let t69551 = t5721 * t13853;
    (t69539, t69541, t69544, t69546, t69548, t69551)
}
