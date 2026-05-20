//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1197;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta334<F: Float>(t40736: F, t9467: F, t9879: F, t2374: F, t39519: F, t39503: F, t118: F, t2375: F, t2448: F, t39391: F, t761: F, t2427: F, t9926: F, t2531: F, t9722: F, t2379: F, t39483: F, t40727: F, t40730: F, t40732: F, t40734: F, t4314: F, t9470: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40737, t40739, t40741, t40743, t40746, t40748, t40750) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1197::<F>(t40736, t9467, t9879, t2374, t39519, t39503, t118, t2375, t2448, t39391, t761, t2427, t9926);
        let (t40755, t40756) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1198::<F>(t2531, t9722, t2379, t39483, t40727, t40730, t40732, t40734, t40737, t40739, t40741, t40743, t40746, t40748, t40750, t4314, t9470);
    (t40737, t40739, t40741, t40743, t40746, t40748, t40750, t40755, t40756)
}
