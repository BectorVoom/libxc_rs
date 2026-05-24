//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 614/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk614<F: Float>(t7372: F, t7377: F, t7381: F, t699: F, t833: F, t739: F, t7405: F, t7412: F, t7422: F, t7425: F, t7436: F, t7440: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8075 = F::cast_from(0.85129199786595678799e-5_f64) * t7372;
    let t8076 = F::cast_from(0.1702583995731913576e-4_f64) * t7377;
    let t8077 = F::cast_from(0.5107751987195740728e-4_f64) * t7381;
    let t8078 = t699 * t833;
    let t8079 = t739 * t8078;
    let t8080 = F::cast_from(0.59871208509319042821e-1_f64) * t8079;
    let t8087 = F::cast_from(0.1702583995731913576e-4_f64) * t7405;
    let t8088 = F::cast_from(0.23942587439980034662e-4_f64) * t7412;
    let t8090 = F::cast_from(0.1702583995731913576e-4_f64) * t7422;
    let t8091 = F::cast_from(0.5107751987195740728e-4_f64) * t7425;
    let t8093 = F::cast_from(0.85129199786595678799e-5_f64) * t7436;
    let t8095 = F::cast_from(0.11974241701863808564e0_f64) * t7440;
    (t8075, t8076, t8077, t8078, t8080, t8087, t8088, t8090, t8091, t8093, t8095)
}
