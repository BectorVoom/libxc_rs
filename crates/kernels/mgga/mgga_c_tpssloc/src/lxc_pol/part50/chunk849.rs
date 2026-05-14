//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 849/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk849<F: Float>(t1509: F, t236: F, t23110: F, t232: F, t23109: F, t1898: F, t4162: F, t249: F, t1496: F, t23069: F, t4257: F, t6621: F, t1512: F, t23041: F, t4166: F, t6613: F) -> (F, F, F, F, F, F) {
    let t25130 = t236 * t1509;
    let t25132 = t23110 * t25130 * t232;
    let t25133 = t23109 * t25132;
    let t25135 = t4162 * t1898;
    let t25136 = t25135 * t249;
    let t25140 = t23069 * t1496;
    let t25142 = t6621 * t4257;
    let t25144 = t23041 * t1512;
    let t25146 = t4166 * t6613;
    (t25133, t25136, t25140, t25142, t25144, t25146)
}
