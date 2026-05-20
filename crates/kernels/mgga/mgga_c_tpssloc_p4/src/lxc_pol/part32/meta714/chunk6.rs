//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2249/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2249<F: Float>(t16662: F, t1894: F, t236: F, t6591: F, t5568: F, t81956: F, t28389: F, t81963: F, t81764: F, t81789: F, t81808: F, t87234: F, t87248: F, t87256: F, t87263: F, t87271: F, t87273: F, t92597: F, t98690: F, t98694: F, t98696: F, t98701: F, t98703: F) -> F {
    let t98707 = t6591 * t1894 * t236 * t16662;
    let t98709 = t81956 * t5568;
    let t98711 = t81963 * t28389;
    let t98713 = -F::new(7.0) / F::new(2304.0) * t98690 - t87234 - F::new(119.0) / F::new(1728.0) * t81764 - t92597 + t87248 + t87256 + t87263 - F::cast_from(0.31625325607076639503e-2_f64) * t81789 + F::new(7.0) / F::new(144.0) * t98694 + F::cast_from(0.84782787797694820792e-2_f64) * t98696 - F::new(119.0) / F::new(6912.0) * t81808 - t87271 + t87273 + F::cast_from(0.40372756094140390854e-3_f64) * t98701 - t98703 / F::new(48.0) - F::cast_from(0.12111826828242117256e-2_f64) * t98707 - F::new(7.0) / F::new(48.0) * t98709 - F::cast_from(0.59347951458386374554e-1_f64) * t98711;
    t98713
}
