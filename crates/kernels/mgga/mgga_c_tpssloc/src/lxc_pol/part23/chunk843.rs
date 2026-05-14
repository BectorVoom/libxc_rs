//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 843/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk843<F: Float>(t12365: F, t1827: F, t12418: F, t820: F, t12289: F, t242: F, t1336: F, t3789: F, t5234: F, t3798: F, t3804: F, t1824: F, t3792: F, t12345: F, t1831: F, t3865: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16211 = t12365 * t1827;
    let t16224 = t12418 * t820;
    let t16232 = t12289 * t242;
    let t16233 = t1336 * t16232;
    let t16285 = t5234 * t3789;
    let t16288 = t5234 * t3798;
    let t16305 = t3804 * t820;
    let t16311 = t1824 * t3792;
    let t16317 = t12345 * t1831;
    let t16336 = t5234 * t3865;
    (t16211, t16224, t16233, t16285, t16288, t16305, t16311, t16317, t16336)
}
