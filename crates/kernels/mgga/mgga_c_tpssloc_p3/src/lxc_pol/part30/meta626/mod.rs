//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2028;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta626<F: Float>(t7496: F, t776: F, t81792: F, t87202: F, t23109: F, t23110: F, t232: F, t236: F, t4233: F, t25132: F, t81876: F, t131: F, t6598: F, t9537: F, t225: F, t2627: F, t25093: F, t1512: F, t81807: F, t81824: F, t23041: F, t4236: F, t23040: F, t4166: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87206, t87212, t87213, t87229) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2028::<F>(t7496, t776, t81792, t87202, t23109, t23110, t232, t236, t4233, t25132, t81876, t131, t6598, t9537);
        let (t87230, t87234, t87243, t87248, t87256, t87261) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2029::<F>(t225, t2627, t236, t25093, t87229, t1512, t81807, t81824, t23041, t4236, t23040, t4166);
    (t87206, t87212, t87213, t87229, t87230, t87234, t87243, t87248, t87256, t87261)
}
