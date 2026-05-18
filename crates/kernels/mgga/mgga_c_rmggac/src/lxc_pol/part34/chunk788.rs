//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 788/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk788<F: Float>(t14084: F, t38839: F, t38844: F, t14091: F, t27: F, t8430: F, t16069: F, t69609: F, t8435: F, t16074: F, t15411: F, t68761: F) -> (F, F, F, F, F, F, F) {
    let t74193 = t14084 * t38839;
    let t74195 = t14084 * t38844;
    let t74197 = t14091 * t38839;
    let t74199 = t14091 * t38844;
    let t74201 = t27 * t8430;
    let t74203 = t69609 * t16069 * t74201;
    let t74205 = t27 * t8435;
    let t74207 = t69609 * t16074 * t74205;
    let t74209 = t68761 * t15411;
    (t74193, t74195, t74197, t74199, t74203, t74207, t74209)
}
