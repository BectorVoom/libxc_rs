//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1115/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1115<F: Float>(t2374: F, t39503: F, t118: F, t2375: F, t2448: F, t39391: F, t761: F, t2427: F, t9926: F, t2531: F, t9722: F, t2379: F, t39483: F, t40727: F, t40730: F, t40732: F, t40734: F, t40737: F, t40739: F, t40741: F, t4314: F, t9470: F) -> (F, F, F, F, F, F) {
    let t40743 = 0.12842595503380418954e1 * t2374 * t39503;
    let t40745 = t2448 * t118 * t2375;
    let t40746 = 0.65061487801810439052e-1 * t40745;
    let t40748 = 0.35089341735807877242e1 * t761 * t39391;
    let t40750 = 16.0 * t2427 * t9926;
    let t40754 = t2531 * t9722;
    let t40755 = 0.4155806185363551302e3 * t40754;
    let t40756 = -36.0 * t2379 * t4314 * t9470 + t39483 + t40727 + t40730 - t40732 - t40734 + t40737 - t40739 - t40741 - t40743 + t40746 + t40748 + t40750 + t40755;
    (t40743, t40746, t40748, t40750, t40755, t40756)
}
