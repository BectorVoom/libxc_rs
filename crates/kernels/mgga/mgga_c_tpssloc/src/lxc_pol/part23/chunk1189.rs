//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1189/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1189<F: Float>(t20816: F, t4205: F, t67230: F, t67243: F, t58972: F, t67463: F, t17116: F, t1877: F, t2522: F, t28248: F, t39549: F, t39563: F, t39585: F, t39590: F, t40799: F, t40801: F, t40803: F, t5664: F, t59564: F) -> (F, F, F, F, F, F) {
    let t75939 = 16.0 * t4205 * t20816;
    let t75940 = 144.0 * t67230;
    let t75941 = 144.0 * t67243;
    let t75942 = 0.65061487801810439052e-1 * t58972;
    let t75943 = 16.0 * t67463;
    let t75947 = -36.0 * t17116 * t2522 * t28248 + 12.0 * t1877 * t5664 * t59564 + t39549 + t39563 - t39585 + t39590 + t40799 + t40801 - t40803 + t75939 + t75940 + t75941 + t75942 + t75943;
    (t75939, t75940, t75941, t75942, t75943, t75947)
}
