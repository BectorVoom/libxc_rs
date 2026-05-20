//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2251/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2251<F: Float>(t16123: F, t2002: F, t559: F, t80920: F, t80922: F, t80943: F, t80957: F, t80959: F, t80971: F, t80989: F, t80992: F, t80998: F, t81007: F, t91394: F, t91398: F, t91400: F, t91403: F, t91404: F, t91406: F, t91413: F) -> F {
    let t91416 = t16123 * t2002 * t559;
    let t91418 = -F::new(119.0) / F::new(6912.0) * t91394 + F::cast_from(0.14130464632949136799e-2_f64) * t80920 + F::cast_from(0.14130464632949136799e-2_f64) * t80922 - F::new(35.0) / F::new(216.0) * t91398 - F::cast_from(0.67826230238155856634e-1_f64) * t91400 + t91403 + F::cast_from(0.16956557559538964158e-1_f64) * t91404 - t91406 - F::cast_from(0.28260929265898273598e-2_f64) * t80943 - t80957 - F::cast_from(0.16956557559538964159e-1_f64) * t80959 + t80971 + F::new(7.0) / F::new(2304.0) * t80989 + F::new(7.0) / F::new(1152.0) * t80992 - F::new(7.0) / F::new(1152.0) * t80998 + F::new(7.0) / F::new(2304.0) * t81007 + t91413 / F::new(192.0) + t91416 / F::new(1536.0);
    t91418
}
