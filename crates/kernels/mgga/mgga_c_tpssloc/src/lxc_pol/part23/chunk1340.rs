//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1340/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1340<F: Float>(t491: F, t79018: F, t11881: F, t11883: F, t11888: F, t15027: F, t1729: F, t22349: F, t22358: F, t22368: F, t22369: F, t22375: F, t22387: F, t3508: F, t3610: F, t44753: F, t44754: F, t44785: F, t44786: F, t470: F, t493: F, t5064: F, t53592: F, t53613: F, t6224: F, t6256: F, t6260: F, t6739: F, t79391: F, t79410: F) -> (F, F) {
    let t79473 = t491 * t79018;
    let t79524 = -36.0 * t11888 * t3508 * t6224 * t6260 * t6739 + 24.0 * t11881 * t11883 * t79410 + 24.0 * t22368 * t3610 * t6256 + 14.0 * t44753 * t44754 * t79473 - t44785 * t44786 * t79473 + t470 * t493 * t79391 + 24.0 * t15027 * t22369 + 4.0 * t1729 * t22375 + 4.0 * t22349 * t53592 + 24.0 * t22358 * t53613 + 4.0 * t22387 * t5064;
    (t79473, t79524)
}
