//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2544/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2544<F: Float>(t1113: F, t136: F, t71189: F, t71201: F, t71191: F, t71195: F, t71199: F, t71468: F, t71470: F, t71472: F, t71474: F, t71477: F, t71480: F, t71483: F) -> (F, F, F) {
    let t71486 = t136 * t1113 * t71189;
    let t71489 = t136 * t1113 * t71201;
    let t71494 = F::cast_from(0.247573125e0_f64) * t71468 - F::cast_from(0.24528888888888888889e-1_f64) * t71470 + F::new(0.11038e0) * t71472 - F::new(0.33114e0) * t71474 + F::new(0.16557e0) * t71477 - F::new(0.82785e-1) * t71480 - F::new(0.82785e-1) * t71483 + F::new(0.49671e0) * t71486 + F::new(0.49671e0) * t71489 + F::new(0.181155e1) * t71191 - F::new(0.36231e1) * t71195 - F::new(0.72462e1) * t71199;
    (t71486, t71489, t71494)
}
