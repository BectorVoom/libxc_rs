//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1212/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1212<F: Float>(t107183: F, t107186: F, t107189: F, t107198: F, t107205: F, t84536: F, t84555: F, t84558: F, t91394: F, t91398: F, t91400: F, t97394: F, t97400: F, t97402: F, t97404: F, t97427: F, t97431: F, t97439: F, t97444: F, t97463: F) -> F {
    let t107860 = F::new(0.24223653656484234512e-2) * t107183 - t84536 - F::new(0.40372756094140390853e-3) * t107186 + F::new(3.0) / F::new(8.0) * t107189 + F::new(7.0) / F::new(24.0) * t97394 - F::new(0.16956557559538964158e-1) * t97400 - F::new(119.0) / F::new(1152.0) * t91394 - F::new(7.0) / F::new(8.0) * t97402 - F::new(0.35608770875031824732e0) * t97404 - F::new(0.84782787797694820791e-2) * t97427 + F::new(0.12111826828242117256e-2) * t97431 - t107198 / F::new(256.0) + F::new(0.50869672678616892474e-1) * t97439 - F::new(35.0) / F::new(36.0) * t91398 - F::new(0.4069573814289351398e0) * t91400 + F::new(0.84782787797694820791e-2) * t97444 + t107205 / F::new(768.0) - t84555 + t84558 + F::new(0.84782787797694820791e-2) * t97463;
    t107860
}
