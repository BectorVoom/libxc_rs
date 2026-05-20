//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 821/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk821<F: Float>(t22692: F, t3851: F, t7208: F, t22717: F, t22725: F, t1332: F, t1336: F, t2089: F, t22697: F, t22701: F, t22707: F, t22721: F, t22728: F, t22730: F, t3773: F, t3777: F, t7209: F, t7211: F) -> F {
    let t24099 = F::cast_from(0.16449340668482264365e-1_f64) * t22692;
    let t24103 = t7208 * t3851;
    let t24108 = F::cast_from(0.12793931631041761173e0_f64) * t22717;
    let t24110 = F::cast_from(0.52089578783527170489e-1_f64) * t22725;
    let t24115 = -t24099 + t3773 * t2089 + F::new(2.0) * t1332 * t7211 - t1336 * t24103 - F::cast_from(0.3289868133696452873e-1_f64) * t22697 - F::cast_from(0.16449340668482264365e-1_f64) * t22701 + F::cast_from(0.16449340668482264365e-1_f64) * t22707 + t24108 + F::cast_from(0.16449340668482264365e-1_f64) * t22721 + t24110 - F::cast_from(0.16449340668482264365e-1_f64) * t22728 - F::cast_from(0.76763589786250567036e-1_f64) * t22730 - F::new(2.0) * t3777 * t7209;
    t24115
}
