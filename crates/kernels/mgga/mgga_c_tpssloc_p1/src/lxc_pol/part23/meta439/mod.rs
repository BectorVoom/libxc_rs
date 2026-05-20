//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta439<F: Float>(t3447: F, t4904: F, t64779: F, t15402: F, t21749: F, t22398: F, t225: F, t1243: F, t72361: F, t22334: F, t22337: F, t22328: F) -> (F, F, F, F, F, F, F) {
        let (t73535, t73541, t73613, t73630, t73856, t73891, t73900) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1283::<F>(t3447, t4904, t64779, t15402, t21749, t22398, t225, t1243, t72361, t22334, t22337, t22328);
    (t73535, t73541, t73613, t73630, t73856, t73891, t73900)
}
