//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 735/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk735<F: Float>(t154: F, t9576: F, t2588: F, t21: F, t59: F, t207: F, t795: F, t4127: F, t787: F, t9526: F, t9529: F, t9540: F, t9542: F, t9544: F, t9547: F, t9552: F, t9556: F, t9559: F, t9561: F, t9566: F, t9572: F, t9574: F) -> (F, F, F) {
    let t9577 = t9576 * t154;
    let t9579 = F::new(0.99999999999999999997e-2) * t9577 * t2588;
    let t9580 = t59 * t21;
    let t9583 = F::new(0.16435185185185185185e-1) * t9580 * t207 * t795;
    let t9584 = F::new(0.49999999999999999998e-2) * t9526 - F::new(0.16666666666666666666e-2) * t787 * t9529 - t9540 - F::new(0.38888888888888888888e-1) * t9542 + F::new(0.11666666666666666666e-1) * t9544 - F::new(0.15833333333333333333e-1) * t9547 - F::new(0.74999999999999999997e-2) * t9552 + F::new(0.24999999999999999999e-2) * t9556 - F::new(0.19999999999999999999e-1) * t9559 * t9561 + F::new(0.14999999999999999999e-1) * t4127 * t9566 - t9572 - F::new(0.34999999999999999998e-1) * t9574 + t9579 - t9583;
    (t9577, t9580, t9584)
}
