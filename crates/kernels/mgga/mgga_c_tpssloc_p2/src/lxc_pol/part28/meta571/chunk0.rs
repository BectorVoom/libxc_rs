//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1851/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1851<F: Float>(t4250: F, t81749: F, t23145: F, t4166: F, t2649: F, t22690: F, t234: F, t7496: F, t776: F, t81792: F, t23109: F, t23110: F, t232: F, t236: F, t4233: F) -> (F, F, F, F, F) {
    let t87197 = t81749 * t4250;
    let t87199 = t4166 * t23145;
    let t87200 = t87199 * t2649;
    let t87202 = t22690 * t234;
    let t87205 = t81792 * t87202 * t7496 * t776;
    let t87211 = t23109 * t23110 * t236 * t4233 * t232;
    (t87197, t87200, t87202, t87205, t87211)
}
