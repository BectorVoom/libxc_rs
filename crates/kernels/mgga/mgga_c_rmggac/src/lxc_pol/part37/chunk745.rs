//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 745/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk745<F: Float>(t2228: F, t265: F, t739: F, t69108: F, t69114: F, t14512: F, t7269: F, t14509: F, t7279: F, t797: F, t838: F, t326: F, t8264: F) -> (F, F, F, F, F, F, F, F, F) {
    let t71835 = t2228 * t265;
    let t71836 = t739 * t71835;
    let t71852 = F::cast_from(0.10492326631435615411e0_f64) * t69108;
    let t71854 = F::cast_from(0.66671395154821946452e-1_f64) * t69114;
    let t71863 = t14512 * t7269;
    let t71871 = t14509 * t7279;
    let t71876 = t797 * t2228;
    let t71882 = t838 * t2228;
    let t71887 = t326 * t8264;
    (t71835, t71836, t71852, t71854, t71863, t71871, t71876, t71882, t71887)
}
