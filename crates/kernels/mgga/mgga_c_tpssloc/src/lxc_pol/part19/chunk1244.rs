//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1244/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1244<F: Float>(t10997: F, t135: F, t973: F, t10480: F, t10483: F, t248: F, t3101: F, t10876: F, t10877: F, t10883: F, t10884: F, t10473: F, t361: F, t363: F, t42342: F, t42345: F) -> (F, F, F, F, F, F) {
    let t43273 = t973 * t135 * t10997;
    let t43277 = t10480 * t248 * t3101 * t10483;
    let t43281 = t10876 * t248 * t3101 * t10877;
    let t43285 = t10883 * t248 * t3101 * t10884;
    let t43288 = 1.0 / t10473 / t361;
    let t43291 = t42342 * t43288 * t363 * t42345;
    (t43273, t43277, t43281, t43285, t43288, t43291)
}
