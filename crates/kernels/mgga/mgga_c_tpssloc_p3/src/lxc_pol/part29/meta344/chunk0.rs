//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1407/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1407<F: Float>(t3732: F, t782: F, t3736: F, t1365: F, t154: F, t205: F, t116: F, t547: F, t1307: F, t212: F, t2586: F, t535: F, t9534: F, t9538: F) -> (F, F, F, F, F, F, F) {
    let t12211 = t782 * t3732;
    let t12212 = t12211 * t3736;
    let t12214 = t154 * t1365;
    let t12215 = t205 * t12214;
    let t12225 = t547 * t116;
    let t12226 = t212 * t1307;
    let t12227 = t12225 * t12226;
    let t12228 = t2586 * t12227;
    let t12236 = F::cast_from(0.13888888888888888889e-3_f64) * t9534 * t535 * t9538;
    (t12211, t12212, t12214, t12215, t12225, t12228, t12236)
}
