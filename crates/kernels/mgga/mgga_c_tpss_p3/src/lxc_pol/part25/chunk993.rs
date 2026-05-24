//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 993/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk993<F: Float>(t13698: F, t4415: F, t4417: F, t10117: F, t5389: F, t10089: F, t1232: F, t13685: F, t12816: F, t1640: F, t4478: F, t12822: F, t12823: F, t5387: F) -> (F, F, F, F, F, F, F) {
    let t13700 = t4415 * t13698 * t4417;
    let t13703 = t10117 * t5389;
    let t13705 = t10089 * t1232;
    let t13707 = t4415 * t13685 * t13705;
    let t13711 = t4415 * t13685 * t4417;
    let t13715 = t12816 * t1640 * t4478;
    let t13719 = t12822 * t12823 * t5387;
    (t13700, t13703, t13705, t13707, t13711, t13715, t13719)
}
