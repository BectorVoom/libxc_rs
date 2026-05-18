//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 767/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk767<F: Float>(t14117: F, t68448: F, t73732: F, t73737: F, t15205: F, t69755: F, t68357: F, t73825: F, t14123: F, t24985: F, t3113: F, t3116: F, t8518: F) -> (F, F, F, F, F) {
    let t73854 = t68448 * t14117 * t73732;
    let t73857 = t68448 * t14117 * t73737;
    let t73862 = t69755 * t15205;
    let t73865 = t68357 * t14117 * t73825;
    let t73871 = t3113 * t24985 * t3116 * t14123 * t14117 * t8518;
    (t73854, t73857, t73862, t73865, t73871)
}
