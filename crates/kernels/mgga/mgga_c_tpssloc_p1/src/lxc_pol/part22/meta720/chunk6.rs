//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2340/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2340<F: Float>(t20994: F, t2563: F, t13251: F, t13262: F, t16816: F, t16836: F, t16845: F, t16893: F, t16969: F, t20908: F, t2623: F, t4178: F, t4180: F, t4182: F, t46875: F, t46876: F, t58705: F, t58709: F, t58723: F, t58731: F, t58735: F, t67607: F) -> F {
    let t67920 = t2563 * t20994;
    let t67926 = F::new(3.0) / F::new(512.0) * t16836 * t16845 + t13251 * t16969 / F::new(128.0) + t46875 + t16836 * t16893 / F::new(512.0) - F::new(3.0) / F::new(256.0) * t13262 * t4180 * t67607 * t16816 + F::new(7.0) / F::new(1536.0) * t4178 * t4180 * t67607 * t4182 - F::new(35.0) / F::new(192.0) * t58705 - F::new(35.0) / F::new(384.0) * t58709 - F::new(119.0) / F::new(4608.0) * t58723 + F::new(7.0) / F::new(768.0) * t58731 + F::new(7.0) / F::new(144.0) * t67920 + F::new(595.0) / F::new(3456.0) * t46876 - F::new(7.0) / F::new(384.0) * t58735 - t2623 * t20908 / F::new(768.0);
    t67926
}
