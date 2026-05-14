//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 639/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk639<F: Float>(t2251: F, t605: F, t6489: F, t9239: F, t2241: F, t72: F, t79: F, t2240: F, t608: F, t1864: F, t645: F, t1863: F, t9231: F, t1860: F, t1865: F, t22490: F, t22493: F, t22513: F, t22516: F, t22519: F, t22523: F, t22527: F, t22531: F, t22534: F, t6486: F, t6490: F, t6492: F, t6495: F, t6506: F, t6510: F) -> (F, F, F, F, F) {
    let t22537 = t605 * t2251;
    let t22544 = t9239 * t6489;
    let t22546 = t72 * t79 * t2241;
    let t22549 = t2240 * t608;
    let t22550 = t1864 * t645;
    let t22551 = t1863 * t22550;
    let t22554 = t9231 * t6489;
    let t22557 = -t1860 * t22490 / 6.0 - t22493 * t1865 / 6.0 - t6486 * t6506 / 3.0 - t6486 * t6510 / 3.0 - t1860 * t22513 / 6.0 - t1860 * t22516 / 3.0 + 2.0 / 3.0 * t22519 * t1865 + 5.0 / 3.0 * t22523 * t6492 + 5.0 / 3.0 * t6490 * t22527 + 5.0 / 6.0 * t6490 * t22531 + t22534 * t1865 / 3.0 + t22537 * t1865 / 3.0 + 2.0 / 3.0 * t6495 * t6506 + 2.0 / 3.0 * t6495 * t6510 - 5.0 * t22544 * t22546 - 10.0 / 3.0 * t22549 * t22551 + 5.0 / 3.0 * t22554 * t6492;
    (t22537, t22546, t22549, t22550, t22557)
}
