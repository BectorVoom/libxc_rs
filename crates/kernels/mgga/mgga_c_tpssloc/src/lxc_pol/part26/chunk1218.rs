//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1218/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1218<F: Float>(t7325: F, t86154: F, t10469: F, t1209: F, t478: F, t11720: F, t3032: F, t11713: F, t11717: F, t24727: F, t11708: F, t24732: F, t7337: F, t11651: F, t24733: F, t11797: F, t7345: F) -> (F, F, F, F, F, F, F, F) {
    let t86155 = t86154 * t7325;
    let t86157 = t10469 * t1209 * t478;
    let t86158 = t11720 * t3032;
    let t86164 = t11713 * t24727 * t11717;
    let t86167 = t11708 * t24732;
    let t86171 = t11713 * t7337 * t11717;
    let t86174 = t24733 * t11651;
    let t86176 = t7345 * t11797;
    (t86155, t86157, t86158, t86164, t86167, t86171, t86174, t86176)
}
