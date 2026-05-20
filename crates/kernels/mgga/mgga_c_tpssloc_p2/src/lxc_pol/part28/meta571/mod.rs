//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1851;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta571<F: Float>(t4250: F, t81749: F, t23145: F, t4166: F, t2649: F, t22690: F, t234: F, t7496: F, t776: F, t81792: F, t23109: F, t23110: F, t232: F, t236: F, t4233: F, t25132: F, t81876: F, t13336: F, t1898: F, t249: F, t23047: F, t2635: F, t1516: F, t81766: F, t23127: F, t4261: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87197, t87200, t87202, t87205, t87211) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1851::<F>(t4250, t81749, t23145, t4166, t2649, t22690, t234, t7496, t776, t81792, t23109, t23110, t232, t236, t4233);
        let (t87213, t87216, t87219, t87222, t87224) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1852::<F>(t25132, t81876, t13336, t1898, t249, t23047, t4166, t2635, t1516, t81766, t23127, t4261);
    (t87197, t87200, t87202, t87205, t87211, t87213, t87216, t87219, t87222, t87224)
}
