//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 982/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk982<F: Float>(t3267: F, t5415: F, t10081: F, t5383: F, t124: F, t13671: F, t762: F, t12817: F, t12822: F, t12828: F, t13698: F, t3273: F, t3275: F, t1233: F, t4415: F, t12863: F, t5387: F) -> (F, F, F, F, F, F, F) {
    let t13725 = t3267 * t5415;
    let t13727 = t10081 * t5383;
    let t13730 = t124 * t13671;
    let t13731 = t762 * t13730;
    let t13736 = t12822 * t12828 * t12817;
    let t13741 = t3273 * t13698 * t3275;
    let t13745 = t4415 * t13698 * t1233;
    let t13749 = t3273 * t12863 * t5387;
    (t13725, t13727, t13731, t13736, t13741, t13745, t13749)
}
