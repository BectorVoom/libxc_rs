//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1860/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1860<F: Float>(t26231: F, t26251: F, t26255: F, t26266: F, t26361: F, t26393: F, t26406: F, t26429: F, t26127: F, t2165: F, t4072: F, t671: F, t8103: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t27012 = F::new(7.0) / F::new(1152.0) * t26231;
    let t27019 = F::new(7.0) / F::new(1152.0) * t26251;
    let t27022 = F::new(7.0) / F::new(288.0) * t26255;
    let t27027 = F::new(7.0) / F::new(72.0) * t26266;
    let t27067 = F::cast_from(0.38381794893125283518e-1_f64) * t26361;
    let t27082 = F::cast_from(0.16449340668482264365e-1_f64) * t26393;
    let t27088 = F::cast_from(0.38381794893125283518e-1_f64) * t26406;
    let t27096 = F::cast_from(0.38381794893125283518e-1_f64) * t26429;
    let t27166 = F::new(2.0) / F::new(3.0) * t26127;
    let t27290 = t2165 * t4072;
    let t27293 = t8103 * t671;
    (t27012, t27019, t27022, t27027, t27067, t27082, t27088, t27096, t27166, t27290, t27293)
}
