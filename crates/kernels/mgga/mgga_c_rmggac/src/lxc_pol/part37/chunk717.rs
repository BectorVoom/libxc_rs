//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 717/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk717<F: Float>(t11648: F, t14236: F, t2067: F, t68626: F, t68854: F, t73793: F, t68856: F, t14125: F, t236: F, t68884: F, t8688: F, t1509: F, t68844: F, t201: F, t457: F, t615: F, t68876: F) -> (F, F, F, F, F, F) {
    let t74839 = t14236 * t68626 * t2067 * t11648;
    let t74841 = t73793 * t68854;
    let t74842 = t74841 * t68856;
    let t74846 = t68884 * t14125 * t236 * t8688;
    let t74848 = t236 * t1509;
    let t74850 = t68844 * t14125 * t74848;
    let t74856 = t68876 * t14125 * t236 * t615 * t457 * t201;
    (t74839, t74842, t74846, t74848, t74850, t74856)
}
