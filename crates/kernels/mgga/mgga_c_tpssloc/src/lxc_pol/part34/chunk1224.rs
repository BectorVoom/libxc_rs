//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1224/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1224<F: Float>(t105278: F, t105282: F, t105286: F, t105288: F, t105290: F, t105292: F, t105294: F, t105296: F, t105299: F, t105304: F, t84857: F, t84859: F, t87213: F, t87243: F, t98618: F, t98647: F, t98690: F, t98694: F, t98696: F) -> F {
    let t108249 = -F::new(7.0) / F::new(48.0) * t98618 + F::new(0.12111826828242117256e-2) * t98647 - t84857 + F::new(0.72670960969452703536e-2) * t105278 + F::new(0.72670960969452703536e-2) * t105282 - F::new(0.14534192193890540707e-1) * t105286 + t105288 / F::new(64.0) + t105290 / F::new(32.0) + t105292 / F::new(64.0) + t105294 / F::new(128.0) - t105296 / F::new(256.0) + t84859 + t105299 / F::new(768.0) + F::new(0.10093189023535097713e-3) * t87213 - F::new(7.0) / F::new(384.0) * t98690 - F::new(119.0) / F::new(1152.0) * t87243 - t105304 / F::new(32.0) + F::new(7.0) / F::new(24.0) * t98694 + F::new(0.50869672678616892474e-1) * t98696;
    t108249
}
