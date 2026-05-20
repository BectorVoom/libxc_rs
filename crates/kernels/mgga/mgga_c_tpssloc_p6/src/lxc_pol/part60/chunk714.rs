//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 714/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk714<F: Float>(t212: F, t252: F, t6554: F, t23171: F, t23030: F, t6563: F, t1883: F, t23012: F, t213: F, t225: F) -> (F, F, F, F, F, F, F, F) {
    let t23228 = t212 * t252;
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23231 = F::cast_from(0.82246703342411321824e-2_f64) * t23230;
    let t23251 = t23030 * t6563;
    let t23252 = F::cast_from(0.26044789391763585244e-1_f64) * t23251;
    let t23261 = t23012 * t1883;
    let t23262 = F::cast_from(0.63969658155208805863e-1_f64) * t23261;
    let t23270 = t213 * t252 * t225;
    (t23228, t23230, t23231, t23251, t23252, t23261, t23262, t23270)
}
