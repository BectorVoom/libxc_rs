//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1557/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1557<F: Float>(t761: F, t9919: F, t2531: F, t2535: F, t2427: F, t2430: F, t32: F, t717: F, t2244: F, t751: F, t2658: F, t2617: F, t2629: F) -> (F, F, F, F, F, F, F) {
    let t9921 = F::cast_from(0.35089341735807877242e1_f64) * t761 * t9919;
    let t9922 = t2531 * t2535;
    let t9924 = t2427 * t2430;
    let t9929 = t32 * t717;
    let t9932 = t751 * t2244;
    let t9933 = t2658 * t9932;
    let t9967 = t2617 * t2629;
    (t9921, t9922, t9924, t9929, t9932, t9933, t9967)
}
