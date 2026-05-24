//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 709/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk709<F: Float>(t9818: F, t9820: F, t530: F, t9343: F, t2211: F, t6557: F, t884: F, t1763: F, t8041: F, t1356: F, t9827: F, t9832: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10302 = F::cast_from(0.2553875993597870364e-4_f64) * t9818;
    let t10303 = F::cast_from(0.1702583995731913576e-4_f64) * t9820;
    let t10304 = t530 * t9343;
    let t10305 = F::new(0.4726e1) * t10304;
    let t10306 = t2211 * t6557;
    let t10307 = t884 * t10306;
    let t10308 = F::cast_from(0.23948483403727617128e0_f64) * t10307;
    let t10309 = t8041 * t1763;
    let t10310 = t1356 * t10309;
    let t10311 = F::cast_from(0.11974241701863808564e0_f64) * t10310;
    let t10312 = F::cast_from(0.85129199786595678799e-5_f64) * t9827;
    let t10313 = F::cast_from(0.13637330827122670865e0_f64) * t9832;
    (t10302, t10303, t10305, t10306, t10308, t10309, t10311, t10312, t10313)
}
