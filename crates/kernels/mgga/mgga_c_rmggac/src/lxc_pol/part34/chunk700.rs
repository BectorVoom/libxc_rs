//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 700/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk700<F: Float>(t15394: F, t68990: F, t68669: F, t3056: F, t3057: F, t8850: F, t8854: F, t8858: F, t8862: F, t15238: F, t5016: F, t2044: F, t558: F, t7273: F, t7554: F, t2084: F, t2145: F, t2367: F, t27: F) -> (F, F, F, F, F, F, F, F, F) {
    let t74503 = t68990 * t15394;
    let t74506 = 0.19863479950205658386e-4 * t68669;
    let t74508 = t3056 * t3057 * t8850;
    let t74511 = t3056 * t3057 * t8854;
    let t74514 = t3056 * t3057 * t8858;
    let t74517 = t3056 * t3057 * t8862;
    let t74520 = 0.5987120850931904282e-1 * t5016 * t15238;
    let t74523 = t7273 * t2044 * t7554 * t558;
    let t74533 = t2145 * t27 * t2084 * t2367;
    (t74503, t74506, t74508, t74511, t74514, t74517, t74520, t74523, t74533)
}
