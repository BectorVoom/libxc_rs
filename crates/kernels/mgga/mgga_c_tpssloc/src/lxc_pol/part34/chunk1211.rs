//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1211/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1211<F: Float>(t107133: F, t107135: F, t107139: F, t107143: F, t107145: F, t107147: F, t107151: F, t107159: F, t107164: F, t107169: F, t107174: F, t107178: F, t84520: F, t84533: F, t91305: F, t91312: F, t91323: F, t91346: F, t97378: F, t97380: F) -> F {
    let t107842 = -t107133 / F::new(192.0) - t107135 / F::new(64.0) - t84520 - F::cast_from(0.13565246047631171326e0_f64) * t107139 - F::cast_from(0.14534192193890540707e-1_f64) * t107143 + t107145 / F::new(32.0) - F::new(5.0) / F::new(64.0) * t107147 + F::new(119.0) / F::new(1152.0) * t91305 - F::cast_from(0.31625325607076639502e-2_f64) * t91312 + t107151 / F::new(128.0) + F::new(7.0) / F::new(384.0) * t97378 - F::new(7.0) / F::new(192.0) * t97380 + F::cast_from(0.60559134141210586279e-3_f64) * t91323 + F::cast_from(0.72670960969452703536e-2_f64) * t107159 + F::cast_from(0.72670960969452703536e-2_f64) * t107164 - F::cast_from(0.50869672678616892475e-1_f64) * t107169 + F::cast_from(0.10093189023535097713e-3_f64) * t91346 - t84533 + F::cast_from(0.72670960969452703536e-2_f64) * t107174 - F::cast_from(0.12111826828242117256e-2_f64) * t107178;
    t107842
}
