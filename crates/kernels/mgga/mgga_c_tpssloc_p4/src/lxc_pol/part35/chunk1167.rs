//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1167/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1167<F: Float>(t26266: F, t26361: F, t26393: F, t26406: F, t26429: F, t26127: F, t2109: F, t26012: F, t33: F, t7973: F, t2240: F, t12571: F, t7245: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27027 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t26266;
    let t27067 = F::cast_from(0.38381794893125283518e-1_f64) * t26361;
    let t27082 = F::cast_from(0.16449340668482264365e-1_f64) * t26393;
    let t27088 = F::cast_from(0.38381794893125283518e-1_f64) * t26406;
    let t27096 = F::cast_from(0.38381794893125283518e-1_f64) * t26429;
    let t27166 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26127;
    let t27298 = t2109 * t26012;
    let t27331 = t33 * t7973;
    let t27332 = t2240 * t27331;
    let t27341 = t12571 * t7245;
    (t27027, t27067, t27082, t27088, t27096, t27166, t27298, t27331, t27332, t27341)
}
