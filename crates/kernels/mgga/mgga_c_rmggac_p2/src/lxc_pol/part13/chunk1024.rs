//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1024/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1024<F: Float>(t8543: F, t8546: F, t8549: F, t8552: F, t9341: F, t9344: F, t7430: F, t7438: F, t8090: F, t8091: F, t8093: F, t8095: F, t8096: F, t8097: F, t8098: F) -> (F, F, F, F, F) {
    let t42435 = F::cast_from(0.11974241701863808564e0_f64) * t8543;
    let t42436 = F::cast_from(0.35922725105591425692e0_f64) * t8546;
    let t42437 = F::cast_from(0.71845450211182851384e0_f64) * t8549;
    let t42438 = F::cast_from(0.17961362552795712846e0_f64) * t8552;
    let t42444 = F::cast_from(0.79828278012425390428e-1_f64) * t9341;
    let t42445 = F::new(0.4726e1) * t9344;
    let t42446 = t8090 + t8091 - F::cast_from(0.79453919800822633544e-4_f64) * t7430 + t8093 + F::cast_from(0.23836175940246790064e-3_f64) * t7438 + t42444 - t8095 - t42445 + t8096 + t8097 + t8098;
    (t42435, t42436, t42437, t42438, t42446)
}
