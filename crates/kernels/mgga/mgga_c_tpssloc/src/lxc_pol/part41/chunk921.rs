//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 921/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk921<F: Float>(t1174: F, t15372: F, t1420: F, t1887: F, t337: F, t11570: F, t3961: F, t1714: F, t4899: F, t11545: F, t60: F, t461: F, t11589: F, t4904: F, t3447: F, t11588: F) -> (F, F, F, F, F, F, F) {
    let t15374 = 0.55555555555555555554e-3 * t1174 * t15372;
    let t15376 = t1420 * t337 * t1887;
    let t15382 = t11570 * t3961;
    let t15390 = t4899 * t1714;
    let t15394 = t60 * t11545;
    let t15395 = t15394 * t461;
    let t15399 = t11589 * t4904;
    let t15401 = 0.18518518518518518518e-3 * t3447 * t15399;
    let t15402 = t11588 * t461;
    (t15374, t15376, t15382, t15390, t15395, t15401, t15402)
}
