//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1265/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1265<F: Float>(t10408: F, t1041: F, t10413: F, t1616: F, t21130: F, t248: F, t3062: F, t3070: F, t3071: F, t42397: F, t50181: F, t5677: F, t5681: F, t5685: F, t5867: F, t5878: F, t62445: F, t62494: F, t62559: F, t62565: F, t70711: F, t70724: F, t70766: F, t70792: F, t70800: F, t70805: F, t76589: F) -> (F,) {
    let t77724 = t70711 / 576.0 + 5.0 / 384.0 * t1041 * t248 * t3062 * t76589 - 5.0 / 10368.0 * t62445 + 5.0 / 2304.0 * t3070 * t10408 * t5677 * t5867 + t10413 * t3071 * t5681 * t5878 / 384.0 + t70724 / 576.0 + 5.0 / 1296.0 * t3070 * t42397 * t21130 * t1616 - t62494 / 1728.0 + t70766 / 1152.0 + t62559 / 108.0 - t62565 / 216.0 + 5.0 / 1728.0 * t70792 - 5.0 / 2304.0 * t10413 * t10408 * t5878 * t5677 + t3070 * t3071 * t5685 * t5867 / 768.0 - t70800 / 576.0 + t70805 / 192.0 + t50181 / 2592.0;
    (t77724,)
}
