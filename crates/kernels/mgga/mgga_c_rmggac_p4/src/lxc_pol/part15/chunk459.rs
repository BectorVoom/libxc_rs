//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 459/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk459<F: Float>(t1411: F, t941: F, t1392: F, t500: F, t4066: F, t4069: F, t1535: F, t446: F, t4116: F, t4120: F, t4124: F, t1004: F, t589: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5321 = t941 * t1411;
    let t5372 = t500 * t1392;
    let t5375 = F::new(48.0) * t4066;
    let t5376 = F::new(80.0) * t4069;
    let t5377 = t1535 * t446;
    let t5382 = F::new(12.0) * t4116;
    let t5385 = F::new(4.0) * t4120;
    let t5388 = F::new(32.0) * t4124;
    let t5389 = t1004 * t589;
    (t5321, t5372, t5375, t5376, t5377, t5382, t5385, t5388, t5389)
}
