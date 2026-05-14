//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1163/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1163<F: Float>(t23384: F, t28481: F, t28691: F, t28705: F, t82431: F, t28681: F, t1054: F, t5943: F, t1921: F, t5914: F, t6688: F, t225: F, t28505: F, t28496: F, t28488: F, t28557: F, t381: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t99151 = t23384 * t28481;
    let t99184 = t23384 * t28691;
    let t99190 = t82431 * t28705;
    let t99205 = t23384 * t28681;
    let t99209 = t1054 * t5943;
    let t99210 = t1921 * t99209;
    let t99214 = t6688 * t5914;
    let t99221 = t28505 * t225;
    let t99230 = t23384 * t28496;
    let t99248 = t28488 * t225;
    let t99273 = t28557 * t381;
    (t99151, t99184, t99190, t99205, t99210, t99214, t99221, t99230, t99248, t99273)
}
