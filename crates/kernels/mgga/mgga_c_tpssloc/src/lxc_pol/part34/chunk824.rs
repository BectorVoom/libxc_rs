//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 824/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk824<F: Float>(t17794: F, t4531: F, t10339: F, t13896: F, t17764: F, t17770: F, t17827: F, t17850: F, t21410: F, t21413: F, t21416: F, t21419: F, t2986: F, t973: F, t17817: F, t17804: F, t4514: F) -> (F, F, F) {
    let t21422 = t4531 * t17794;
    let t21429 = -0.83333333333333333331e-3 * t17827 - 0.22222222222222222221e-2 * t973 * t21410 + 0.11111111111111111111e-2 * t2986 * t21413 - 0.11111111111111111111e-2 * t2986 * t21416 - 0.83333333333333333331e-3 * t2986 * t21419 - 0.83333333333333333331e-3 * t2986 * t21422 - 0.55555555555555555554e-3 * t17764 + 0.27777777777777777777e-3 * t17770 - 0.83333333333333333331e-3 * t17850 + t10339 - 0.18518518518518518518e-3 * t13896;
    let t21430 = t4531 * t17817;
    let t21433 = t17804 * t4514;
    (t21429, t21430, t21433)
}
