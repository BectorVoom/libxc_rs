//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2023/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2023<F: Float>(t80837: F, t84514: F, t84520: F, t91244: F, t91246: F, t91247: F, t93710: F, t93711: F, t93712: F, t93715: F, t93718: F, t97352: F, t97354: F, t97359: F, t97361: F, t97363: F, t97367: F, t97372: F) -> F {
    let t102705 = -t97352 / F::new(192.0) + F::new(5.0) / F::new(192.0) * t97354 + t91244 - t91246 + t91247 - t84514 + F::new(5.0) / F::new(192.0) * t97359 + F::new(5.0) / F::new(96.0) * t97361 - F::new(7.0) / F::new(1152.0) * t97363 - F::cast_from(0.13457585364713463618e-3_f64) * t97367 + F::cast_from(0.67287926823567318088e-4_f64) * t97372 + F::cast_from(0.20186378047070195426e-3_f64) * t80837 - t84520 + t93710 + t93711 + t93712 - t93715 - t93718;
    t102705
}
