//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2310/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2310<F: Float>(t8070: F, t94490: F, t86036: F, t95760: F, t103779: F, t1409: F, t1734: F, t19138: F, t24849: F, t24851: F, t27502: F, t27507: F, t27532: F, t27540: F, t29735: F, t3624: F, t3966: F, t5011: F, t6256: F, t7327: F, t7376: F, t8082: F, t86015: F, t86116: F, t95098: F, t95114: F, t95197: F, t95201: F, t95761: F) -> F {
    let t103830 = t94490 * t8070;
    let t103838 = t86036 * t95760;
    let t103864 = F::cast_from(0.14621636149762012769e-1_f64) * t103830 - F::cast_from(0.54831135561607547883e-2_f64) * t24849 * t7327 * t6256 * t27532 - F::cast_from(0.16449340668482264365e-1_f64) * t95761 * t27540 - F::cast_from(0.3289868133696452873e-1_f64) * t103838 * t95197 + F::cast_from(0.16449340668482264365e-1_f64) * t103838 * t95201 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t86116 * t29735 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t24851 * t3966 * t1734 * t7376 - t95098 - t95114 - F::cast_from(0.43864908449286038306e-1_f64) * t27507 * t27502 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t24851 * t1409 * t5011 * t7376 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t86015 * t103779 - F::cast_from(2.0_f64) * t3624 * t8082 * t19138;
    t103864
}
