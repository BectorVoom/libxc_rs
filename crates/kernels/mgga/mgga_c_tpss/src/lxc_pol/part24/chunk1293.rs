//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1293/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1293<F: Float>(t18450: F, t5410: F, t5383: F, t60695: F, t5415: F, t13795: F, t5728: F, t13800: F, t13858: F, t13856: F, t19469: F, t215: F, t13731: F, t5716: F, t13677: F, t18454: F) -> (F, F, F, F, F, F, F, F, F) {
    let t69531 = t18450 * t5410;
    let t69533 = t60695 * t5383;
    let t69535 = t18450 * t5415;
    let t69537 = t5728 * t13795;
    let t69539 = t5728 * t13800;
    let t69541 = t5728 * t13858;
    let t69544 = t19469 * t215 * t13856;
    let t69546 = t5716 * t13731;
    let t69548 = t18454 * t13677;
    (t69531, t69533, t69535, t69537, t69539, t69541, t69544, t69546, t69548)
}
