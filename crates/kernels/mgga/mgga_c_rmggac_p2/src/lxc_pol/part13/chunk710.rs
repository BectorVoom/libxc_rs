//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 710/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk710<F: Float>(t8694: F, t8696: F, t8698: F, t9499: F, t9040: F, t9060: F, t9062: F, t9075: F, t9079: F, t9083: F, t9091: F, t9650: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10358 = F::cast_from(0.11918087970123395032e-3_f64) * t8694;
    let t10359 = F::cast_from(0.11918087970123395032e-3_f64) * t8696;
    let t10360 = F::cast_from(0.39726959900411316772e-4_f64) * t8698;
    let t10376 = F::new(2.0) * t9499;
    let t10384 = F::cast_from(0.39726959900411316772e-4_f64) * t9040;
    let t10385 = F::cast_from(0.47896966807455234256e0_f64) * t9060;
    let t10386 = F::cast_from(0.3193131120497015617e0_f64) * t9062;
    let t10487 = F::cast_from(0.15965655602485078085e0_f64) * t9075;
    let t10496 = F::cast_from(0.15965655602485078085e0_f64) * t9079;
    let t10503 = F::cast_from(0.1440846329149835838e-2_f64) * t9083;
    let t10504 = F::cast_from(0.39726959900411316772e-4_f64) * t9091;
    let t10508 = F::new(2.0) * t9650;
    (t10358, t10359, t10360, t10376, t10384, t10385, t10386, t10487, t10496, t10503, t10504, t10508)
}
