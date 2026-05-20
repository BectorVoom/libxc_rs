//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1051/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1051<F: Float>(t461: F, t4899: F, t4724: F, t1409: F, t3450: F, t3449: F, t3448: F, t4729: F, t1178: F, t3966: F, t1177: F, t135: F, t1716: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4900 = t4899 * t461;
    let t4901 = t4900 * t4724;
    let t4904 = t3450 * t1409;
    let t4905 = t3449 * t4904;
    let t4908 = t3448 * t461;
    let t4909 = t4908 * t4729;
    let t4912 = t1178 * t3966;
    let t4913 = t1177 * t4912;
    let t4916 = t135 * t1716;
    (t4900, t4901, t4904, t4905, t4908, t4909, t4912, t4913, t4916)
}
