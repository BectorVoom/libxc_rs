//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 493/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk493<F: Float>(t5338: F, t5353: F, t277: F, t1392: F, t500: F, t4066: F, t4069: F, t1535: F, t446: F, t4085: F, t4114: F, t4116: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5354 = t5338 + t5353;
    let t5355 = t277 * t5354;
    let t5372 = t500 * t1392;
    let t5375 = F::new(48.0) * t4066;
    let t5376 = F::new(80.0) * t4069;
    let t5377 = t1535 * t446;
    let t5380 = F::cast_from(0.21687162600603479684e-1_f64) * t4085;
    let t5381 = F::new(40.0) * t4114;
    let t5382 = F::new(12.0) * t4116;
    (t5354, t5355, t5372, t5375, t5376, t5377, t5380, t5381, t5382)
}
