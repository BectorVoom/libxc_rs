//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 708/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk708<F: Float>(t3148: F, t3151: F, t9086: F, t14011: F, t1616: F, t3120: F, t14371: F, t15333: F, t13862: F, t14041: F, t8615: F, t14078: F, t8659: F, t14125: F, t236: F, t68884: F, t8602: F) -> (F, F, F, F, F, F) {
    let t74647 = t9086 * t3148 * t3151;
    let t74650 = t3120 * t14011 * t1616;
    let t74652 = t14371 * t15333;
    let t74655 = t14041 * t13862 * t8615;
    let t74657 = t8659 * t14078;
    let t74662 = t68884 * t14125 * t236 * t8602;
    (t74647, t74650, t74652, t74655, t74657, t74662)
}
