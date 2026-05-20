//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2078/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2078<F: Float>(t9467: F, t9879: F, t2374: F, t39519: F, t39503: F, t118: F, t2375: F, t2448: F, t39391: F, t761: F, t2531: F, t9722: F) -> (F, F, F, F, F, F) {
    let t40738 = t9879 * t9467;
    let t40741 = F::cast_from(0.43374325201206959368e-1_f64) * t2374 * t39519;
    let t40743 = F::cast_from(0.12842595503380418954e1_f64) * t2374 * t39503;
    let t40745 = t2448 * t118 * t2375;
    let t40748 = F::cast_from(0.35089341735807877242e1_f64) * t761 * t39391;
    let t40754 = t2531 * t9722;
    (t40738, t40741, t40743, t40745, t40748, t40754)
}
