//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1234/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1234<F: Float>(t26504: F, t8690: F, t120703: F, t120708: F, t120709: F, t120711: F, t120714: F, t120716: F, t120719: F, t120721: F, t120723: F, t120728: F, t120730: F, t120732: F, t2165: F, t26135: F, t652: F) -> (F, F) {
    let t123235 = t8690 * t26504;
    let t123242 = 3.0 * t120703 + t123235 - t120708 - 2.0 * t120709 - 2.0 * t120711 - 2.0 * t120714 - 2.0 * t120716 - t120719 - t120721 - 2.0 * t120723 - t120728 - t120730 - 2.0 * t120732;
    let t123244 = t652 * t2165 * t26135;
    (t123242, t123244)
}
