//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2051/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2051<F: Float>(t2652: F, t9874: F, t39488: F, t761: F, t2531: F, t9919: F, t9467: F, t9879: F, t2374: F, t39519: F, t39503: F, t39391: F) -> (F, F, F, F, F, F, F) {
    let t40722 = t2652 * t9874;
    let t40732 = F::cast_from(0.6233709278045326953e3_f64) * t761 * t39488;
    let t40733 = t2531 * t9919;
    let t40738 = t9879 * t9467;
    let t40741 = F::cast_from(0.43374325201206959368e-1_f64) * t2374 * t39519;
    let t40743 = F::cast_from(0.12842595503380418954e1_f64) * t2374 * t39503;
    let t40748 = F::cast_from(0.35089341735807877242e1_f64) * t761 * t39391;
    (t40722, t40732, t40733, t40738, t40741, t40743, t40748)
}
