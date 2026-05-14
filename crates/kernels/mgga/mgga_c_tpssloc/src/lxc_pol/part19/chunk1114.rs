//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1114/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1114<F: Float>(t751: F, t9288: F, t9897: F, t2244: F, t2517: F, t2658: F, t39488: F, t761: F, t2531: F, t9919: F, t707: F, t9258: F, t9467: F, t9879: F, t2374: F, t39519: F) -> (F, F, F, F, F, F, F) {
    let t40726 = t9897 * t751 * t9288;
    let t40727 = 96.0 * t40726;
    let t40729 = t2658 * t2517 * t2244;
    let t40730 = 72.0 * t40729;
    let t40732 = 0.6233709278045326953e3 * t761 * t39488;
    let t40733 = t2531 * t9919;
    let t40734 = 0.14035736694323150897e2 * t40733;
    let t40736 = t707 * t751 * t9258;
    let t40737 = 16.0 * t40736;
    let t40738 = t9879 * t9467;
    let t40739 = 0.86748650402413918736e-1 * t40738;
    let t40741 = 0.43374325201206959368e-1 * t2374 * t39519;
    (t40727, t40730, t40732, t40734, t40737, t40739, t40741)
}
