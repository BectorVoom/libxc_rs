//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1277/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1277<F: Float>(t1865: F, t22513: F, t22519: F, t22523: F, t22527: F, t22544: F, t22546: F, t22549: F, t22554: F, t6492: F, t6495: F, t6506: F, t6510: F, t83734: F, t83738: F, t83741: F, t83745: F, t83748: F, t83750: F, t83760: F) -> (F,) {
    let t83766 = -15.0 * t22544 * t83734 - 5.0 * t22549 * t83738 - 15.0 * t83741 * t22546 - 15.0 * t22544 * t83745 + t83748 * t1865 + 5.0 * t83750 * t6492 + 2.0 * t22519 * t6506 + 5.0 * t22554 * t22527 + 2.0 * t22519 * t6510 + 5.0 / 2.0 * t83760 * t6492 + t6495 * t22513 + 5.0 * t22523 * t22527;
    (t83766,)
}
