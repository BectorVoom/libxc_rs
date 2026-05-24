//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 844/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk844<F: Float>(t75123: F, t7720: F, t14020: F, t14117: F, t14123: F, t21052: F, t73712: F, t68357: F, t73717: F, t15394: F, t70548: F, t2060: F, t8794: F) -> (F, F, F, F, F) {
    let t75124 = t7720 * t75123;
    let t75134 = t21052 * t14020 * t14123 * t14117 * t73712;
    let t75137 = t68357 * t14117 * t73717;
    let t75139 = t70548 * t15394;
    let t75141 = t2060 * t8794;
    (t75124, t75134, t75137, t75139, t75141)
}
