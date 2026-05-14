//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 803/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk803<F: Float>(t207: F, t215: F, t9569: F, t2570: F, t782: F, t2573: F, t2690: F, t59: F, t154: F, t2588: F, t21: F, t795: F, t4127: F, t787: F, t9526: F, t9529: F, t9540: F, t9542: F, t9544: F, t9547: F, t9552: F, t9556: F, t9559: F, t9561: F, t9566: F) -> (F, F, F, F) {
    let t9572 = 0.28086419753086419752e-1 * t9569 * t207 * t215;
    let t9573 = t782 * t2570;
    let t9574 = t9573 * t2573;
    let t9576 = t59 * t2690;
    let t9577 = t9576 * t154;
    let t9579 = 0.99999999999999999997e-2 * t9577 * t2588;
    let t9580 = t59 * t21;
    let t9583 = 0.16435185185185185185e-1 * t9580 * t207 * t795;
    let t9584 = 0.49999999999999999998e-2 * t9526 - 0.16666666666666666666e-2 * t787 * t9529 - t9540 - 0.38888888888888888888e-1 * t9542 + 0.11666666666666666666e-1 * t9544 - 0.15833333333333333333e-1 * t9547 - 0.74999999999999999997e-2 * t9552 + 0.24999999999999999999e-2 * t9556 - 0.19999999999999999999e-1 * t9559 * t9561 + 0.14999999999999999999e-1 * t4127 * t9566 - t9572 - 0.34999999999999999998e-1 * t9574 + t9579 - t9583;
    (t9573, t9577, t9580, t9584)
}
