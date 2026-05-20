//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1425/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1425<F: Float>(t5151: F, t67: F, t758: F, t12365: F, t1827: F, t12300: F, t12418: F, t820: F, t1351: F, t1799: F, t12289: F, t242: F) -> (F, F, F, F, F, F) {
    let t16169 = t5151 * t67;
    let t16171 = F::cast_from(0.36622894612013090108e-3_f64) * t16169 * t758;
    let t16211 = t12365 * t1827;
    let t16214 = F::new(7.0) / F::new(2304.0) * t12300 * t1827;
    let t16224 = t12418 * t820;
    let t16225 = t1799 * t1351;
    let t16232 = t12289 * t242;
    (t16171, t16211, t16214, t16224, t16225, t16232)
}
