//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 622/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk622<F: Float>(t7289: F, t7383: F, t7402: F, t7430: F, t7438: F, t7708: F, t7770: F, t7780: F, t7908: F, t7910: F, t7940: F, t2416: F, t7487: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8040 = F::cast_from(0.10909864661698136692e0_f64) * t7289;
    let t8081 = F::cast_from(0.15965655602485078085e0_f64) * t7383;
    let t8086 = F::cast_from(0.39726959900411316772e-4_f64) * t7402;
    let t8092 = F::cast_from(0.39726959900411316772e-4_f64) * t7430;
    let t8094 = F::cast_from(0.11918087970123395032e-3_f64) * t7438;
    let t8173 = F::cast_from(0.3193131120497015617e0_f64) * t7708;
    let t8196 = F::cast_from(0.47896966807455234256e0_f64) * t7770;
    let t8197 = F::cast_from(0.15965655602485078085e0_f64) * t7780;
    let t8221 = F::cast_from(0.39726959900411316772e-4_f64) * t7908;
    let t8222 = F::cast_from(0.11918087970123395032e-3_f64) * t7910;
    let t8304 = F::cast_from(0.39726959900411316772e-4_f64) * t7940;
    let t8328 = t7487 * t2416;
    (t8040, t8081, t8086, t8092, t8094, t8173, t8196, t8197, t8221, t8222, t8304, t8328)
}
