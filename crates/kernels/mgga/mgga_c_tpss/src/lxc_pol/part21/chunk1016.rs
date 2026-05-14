//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1016/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1016<F: Float>(t10980: F, t10986: F, t11003: F, t11005: F, t11006: F, t11010: F, t11015: F, t11020: F, t11024: F, t11028: F, t11033: F, t11037: F, t8605: F, t8607: F, t8616: F, t8618: F, t8687: F) -> (F,) {
    let t11040 = -t8687 - 8.0 / 27.0 * t8616 + 2.0 / 27.0 * t8607 - 2.0 / 9.0 * t8618 + t8605 / 9.0 - 4.0 / 27.0 * t10980 + t11003 - t11005 + t11006 - 10.0 / 27.0 * t11010 + 4.0 / 3.0 * t11015 - 4.0 / 9.0 * t11020 - 2.0 / 9.0 * t11024 - 2.0 * t11028 + 4.0 / 3.0 * t11033 + 2.0 / 3.0 * t11037 - t10986 / 3.0;
    (t11040,)
}
