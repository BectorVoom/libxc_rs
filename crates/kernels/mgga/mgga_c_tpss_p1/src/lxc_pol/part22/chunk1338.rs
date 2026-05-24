//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1338/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1338<F: Float>(t1656: F, t3259: F, t3260: F, t41437: F, t520: F, t1640: F, t3384: F, t19539: F, t5736: F, t3366: F, t12828: F, t3326: F) -> (F, F, F, F, F, F, F) {
    let t65695 = t1656 * t3259;
    let t65696 = t65695 * t3260;
    let t65703 = t41437 * t520;
    let t65711 = t65695 * t520;
    let t65715 = t1640 * t3384;
    let t65719 = t5736 * t19539;
    let t65722 = t1640 * t3366;
    let t65729 = t12828 * t3326;
    (t65696, t65703, t65711, t65715, t65719, t65722, t65729)
}
