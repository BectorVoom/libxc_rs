//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2123/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2123<F: Float>(t87363: F, t13080: F, t23146: F, t242: F, t812: F, t81816: F, t13265: F, t13333: F, t25084: F, t13076: F, t13084: F, t87329: F, t87331: F, t87333: F, t87336: F, t87339: F, t87342: F, t87343: F, t87345: F, t87348: F, t87351: F, t87355: F, t87359: F) -> F {
    let t87364 = F::new(7.0) / F::new(576.0) * t87363;
    let t87365 = t23146 * t13080;
    let t87368 = t812 * t81816 * t242;
    let t87369 = t87368 * t13265;
    let t87371 = t25084 * t13333;
    let t87373 = t23146 * t13076;
    let t87375 = t25084 * t13084;
    let t87377 = -t87329 + t87331 + t87333 - t87336 + t87339 + t87342 - t87343 / F::new(384.0) - F::new(119.0) / F::new(1728.0) * t87345 - t87348 - F::cast_from(0.84782787797694820792e-2_f64) * t87351 - F::cast_from(0.12111826828242117256e-2_f64) * t87355 - F::cast_from(0.12111826828242117256e-2_f64) * t87359 - t87364 - F::new(5.0) / F::new(384.0) * t87365 - t87369 / F::new(256.0) + t87371 / F::new(256.0) - t87373 / F::new(1536.0) - t87375 / F::new(192.0);
    t87377
}
