//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1225/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1225<F: Float>(t105309: F, t105311: F, t105313: F, t105315: F, t105317: F, t105319: F, t105325: F, t105329: F, t105333: F, t105335: F, t105337: F, t105339: F, t105341: F, t105345: F, t105348: F, t84896: F, t84897: F, t98709: F, t98711: F, t98725: F) -> F {
    let t108268 = -t105309 / F::cast_from(256.0_f64) + t105311 / F::cast_from(128.0_f64) - t105313 / F::cast_from(64.0_f64) - t105315 / F::cast_from(192.0_f64) - t105317 / F::cast_from(64.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t105319 - F::cast_from(7.0_f64) / F::cast_from(8.0_f64) * t98709 - F::cast_from(0.35608770875031824732e0_f64) * t98711 - t84896 - t84897 - F::cast_from(0.12111826828242117256e-2_f64) * t105325 + F::cast_from(0.72670960969452703536e-2_f64) * t105329 + F::cast_from(0.24223653656484234512e-2_f64) * t105333 - t105335 / F::cast_from(768.0_f64) - t105337 / F::cast_from(256.0_f64) - t105339 / F::cast_from(256.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t105341 + F::cast_from(0.84782787797694820791e-2_f64) * t98725 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t105345 - F::cast_from(0.24223653656484234512e-2_f64) * t105348;
    t108268
}
