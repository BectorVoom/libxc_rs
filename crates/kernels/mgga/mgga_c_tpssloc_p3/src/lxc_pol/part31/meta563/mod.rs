//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1793;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta563<F: Float>(t81615: F, t7524: F, t81612: F, t81613: F, t4250: F, t81749: F, t23145: F, t4166: F, t22690: F, t234: F, t7496: F, t776: F, t81792: F, t23109: F, t23110: F, t232: F, t236: F, t4233: F, t25132: F, t81876: F, t131: F, t6598: F, t9537: F, t225: F, t2627: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87167, t87177, t87197, t87199, t87202, t87205) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1793::<F>(t81615, t7524, t81612, t81613, t4250, t81749, t23145, t4166, t22690, t234, t7496, t776, t81792);
        let (t87211, t87213, t87229, t87230) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1794::<F>(t23109, t23110, t232, t236, t4233, t25132, t81876, t131, t6598, t9537, t225, t2627);
    (t87167, t87177, t87197, t87199, t87202, t87205, t87211, t87213, t87229, t87230)
}
