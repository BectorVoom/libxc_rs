//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1133/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1133<F: Float>(t28135: F, t6914: F, t28168: F, t562: F, t6347: F, t1799: F, t1834: F, t22704: F, t22705: F, t28167: F, t6330: F, t28163: F) -> (F, F, F, F, F, F, F) {
    let t96937 = t6914 * t28135;
    let t96945 = t6914 * t28168;
    let t96951 = t562 * t6347;
    let t96964 = t1834 * t1799;
    let t96989 = t22704 * t22705 * t28167;
    let t97011 = t562 * t6330;
    let t97026 = t22704 * t22705 * t28163;
    (t96937, t96945, t96951, t96964, t96989, t97011, t97026)
}
