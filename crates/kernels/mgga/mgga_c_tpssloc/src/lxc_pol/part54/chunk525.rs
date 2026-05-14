//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 525/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk525<F: Float>(t4211: F, t758: F, t2431: F, t2532: F, t2653: F, t2417: F, t2423: F, t2426: F, t2486: F, t2518: F, t2530: F, t2537: F, t2538: F, t2665: F, t225: F, t4210: F) -> (F, F, F, F, F) {
    let t4212 = t4211 * t758;
    let t4213 = 0.18311447306006545054e-3 * t4212;
    let t4214 = 4.0 * t2431;
    let t4215 = 0.5848223622634646207e0 * t2532;
    let t4216 = 0.18311447306006545054e-3 * t2653;
    let t4217 = t2417 - t2423 - t2426 - t4213 + t4214 + t2518 - t2530 - t4215 - t2537 + t2538 + t2665 - t4216 - t2486;
    let t4219 = (t4210 + t4217) * t225;
    (t4213, t4214, t4215, t4216, t4219)
}
