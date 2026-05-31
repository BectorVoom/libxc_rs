//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 967/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk967<F: Float>(t30: F, t33: F, t1289: F, t1985: F, t7737: F, t2009: F, t3431: F, t581: F, t1992: F, t3446: F, t555: F, t7622: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t10340 = t7737 * t1289 * t1985;
    let t10343 = t2009 * t3431;
    let t10344 = t10343 * t581;
    let t10347 = t3446 * t1992;
    let t10350 = F::cast_from(2.0_f64) * t555;
    let t10351 = F::cast_from(6.0_f64) * t7622;
    let t10353 = piecewise5::<F>(t31, F::cast_from(0.0_f64), t34, F::cast_from(0.0_f64), t10350 - t10351);
    (t10340, t10344, t10347, t10353)
}
