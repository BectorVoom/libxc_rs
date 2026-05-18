//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1447/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1447<F: Float>(t122448: F, t1352: F, t22633: F, t6976: F, t27074: F, t3807: F, t115399: F, t1799: F, t6637: F, t6888: F, t31618: F, t5187: F) -> (F, F, F, F) {
    let t122518 = t22633 * t6976 * t122448 * t1352;
    let t122522 = t22633 * t6976 * t27074 * t3807;
    let t122526 = t6888 * t6637 * t115399 * t1799;
    let t122530 = t6888 * t6637 * t31618 * t5187;
    (t122518, t122522, t122526, t122530)
}
