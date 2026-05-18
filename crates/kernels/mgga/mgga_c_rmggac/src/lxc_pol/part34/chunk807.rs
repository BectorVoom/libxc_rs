//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 807/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk807<F: Float>(t14226: F, t74497: F, t21719: F, t3352: F, t8811: F, t15394: F, t68990: F, t68669: F, t3056: F, t3057: F, t8850: F, t8854: F) -> (F, F, F, F, F, F) {
    let t74498 = t74497 * t14226;
    let t74501 = t21719 * t3352 * t8811;
    let t74503 = t68990 * t15394;
    let t74506 = F::new(0.19863479950205658386e-4) * t68669;
    let t74508 = t3056 * t3057 * t8850;
    let t74511 = t3056 * t3057 * t8854;
    (t74498, t74501, t74503, t74506, t74508, t74511)
}
