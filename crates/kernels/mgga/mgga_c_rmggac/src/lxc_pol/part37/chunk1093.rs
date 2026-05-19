//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1093/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1093<F: Float>(t75362: F, t75364: F, t75356: F, t75367: F, t75369: F, t75371: F, t75378: F, t75380: F, t75383: F, t78148: F, t78150: F, t78156: F, t78157: F, t78161: F, t78162: F, t78163: F) -> F {
    let t80358 = F::cast_from(0.2419210303588817044e-2_f64) * t75362;
    let t80359 = F::cast_from(0.33868944250243438616e-2_f64) * t75364;
    let t80366 = t78148 - F::cast_from(0.50803416375365157924e-2_f64) * t75356 + t78150 + t80358 - t80359 - F::cast_from(0.68186654135613354324e-2_f64) * t75367 - F::cast_from(0.68186654135613354324e-2_f64) * t75369 + F::cast_from(0.13637330827122670865e-1_f64) * t75371 + t78156 + t78157 + F::cast_from(0.2727466165424534173e-1_f64) * t75378 + F::cast_from(0.2727466165424534173e-1_f64) * t75380 - F::cast_from(0.68186654135613354325e-1_f64) * t75383 + t78161 - t78162 - t78163;
    t80366
}
