//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 479/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk479<F: Float>(t1315: F, t1327: F, t1329: F, t1333: F, t1341: F, t1354: F, t1360: F, t1363: F, t1369: F, t559: F) -> F {
    let t1372 = -t1327 - t1315 * t1329 / F::cast_from(48.0_f64) + t1333 * t559 / F::cast_from(3072.0_f64) - t1341 * t1354 / F::cast_from(3072.0_f64) - t1360 - t1363 * t1369 / F::cast_from(768.0_f64);
    t1372
}
