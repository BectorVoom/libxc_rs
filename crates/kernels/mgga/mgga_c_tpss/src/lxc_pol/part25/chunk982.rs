//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 982/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk982<F: Float>(t10980: F, t10983: F, t10990: F, t14495: F, t14497: F, t14501: F, t14503: F, t14505: F, t14507: F, t8616: F, t8627: F, t14469: F, t8609: F, t128: F) -> (F, F) {
    let t14510 = 0.66437037037037037037e-1 * t14495 + 0.18257037037037037037e-1 * t14497 - 0.13287407407407407408e0 * t8616 - 0.91285185185185185187e-1 * t8627 - 0.10954222222222222222e0 * t14501 + 0.54771111111111111111e-1 * t14503 - 0.19931111111111111111e0 * t14505 + 0.99655555555555555557e-1 * t14507 - 0.26574814814814814815e0 * t10980 + t10983 + t10990;
    let t14516 = t8609 * t14469;
    let t14517 = t128 * t14516;
    (t14510, t14517)
}
