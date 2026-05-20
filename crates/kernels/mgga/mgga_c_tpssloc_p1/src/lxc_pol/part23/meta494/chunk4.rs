//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1523/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1523<F: Float>(t6324: F, t1390: F, t193: F, t20085: F, t39658: F, t39660: F, t39844: F, t39856: F, t40224: F, t40228: F, t40230: F, t40611: F, t5160: F, t533: F, t6463: F, t80112: F, t80113: F, t80114: F, t80115: F, t80116: F, t80489: F, t80521: F) -> F {
    let t80529 = t6324 * t6324;
    let t80534 = t193 * t533 * (t80489 + t80521) * t1390 - t39658 + t39660 + t39844 + F::new(12.0) * t5160 * t20085 * t6463 - t80112 - t80113 - t39856 - t80114 - F::new(6.0) * t193 * t533 * t80529 * t40611 + t40224 + t40228 - t40230 + t80115 - t80116;
    t80534
}
