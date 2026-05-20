//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 249/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk249<F: Float>(t134: F, t340: F, t344: F, t221: F, t339: F, t209: F, t338: F, t39: F) -> (F, F, F, F) {
    let t967 = t134 * t340;
    let t968 = t967 * t344;
    let t969 = t221 * t968;
    let t971 = F::cast_from(0.27777777777777777777e-3_f64) * t339 * t969;
    let t972 = t338 * t209;
    let t973 = t39 * t972;
    (t967, t971, t972, t973)
}
