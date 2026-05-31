//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1896/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1896<F: Float>(t26231: F, t26251: F, t26255: F, t26266: F, t26361: F, t26393: F, t26406: F, t26429: F, t26127: F, t19299: F, t33: F, t22505: F, t22510: F, t5392: F, t5398: F, t6500: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t27012 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t26231;
    let t27019 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t26251;
    let t27022 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t26255;
    let t27027 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t26266;
    let t27067 = F::cast_from(0.38381794893125283518e-1_f64) * t26361;
    let t27082 = F::cast_from(0.16449340668482264365e-1_f64) * t26393;
    let t27088 = F::cast_from(0.38381794893125283518e-1_f64) * t26406;
    let t27096 = F::cast_from(0.38381794893125283518e-1_f64) * t26429;
    let t27166 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26127;
    let t27937 = t19299 * t33;
    let t27948 = F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t22505 * t5392 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6500 * t5398 - t22510;
    (t27012, t27019, t27022, t27027, t27067, t27082, t27088, t27096, t27166, t27937, t27948)
}
