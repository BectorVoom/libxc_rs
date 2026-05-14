//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 587/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk587<F: Float>(t5542: F, t8601: F, t674: F, t8607: F, t8687: F, t3924: F, t623: F, t34760: F, t8450: F, t2185: F, t2338: F, t7556: F, t31: F, t574: F, t640: F, t34795: F, t529: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38350 = t8601 * t5542;
    let t38351 = t38350 * t674;
    let t38354 = t8607 * t5542;
    let t38355 = t38354 * t674;
    let t38471 = t8687 * t5542;
    let t38472 = t38471 * t674;
    let t38495 = t623 * t3924;
    let t38530 = t8450 * t34760;
    let t38638 = t8450 * t2185;
    let t38839 = t2338 * t7556;
    let t38843 = t574 * t31;
    let t38844 = t640 * t38843;
    let t38848 = t34795 * t529;
    (t38350, t38351, t38354, t38355, t38471, t38472, t38495, t38530, t38638, t38839, t38844, t38848)
}
