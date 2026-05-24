//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 756/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk756<F: Float>(t14121: F, t14123: F, t14125: F, t21060: F, t73712: F, t236: F, t495: F, t589: F, t69009: F, t498: F, t68421: F, t68422: F) -> (F, F, F, F, F) {
    let t73714 = t21060 * t14121 * t14123 * t14125 * t73712;
    let t73717 = t236 * t589 * t495;
    let t73719 = t69009 * t14125 * t73717;
    let t73722 = t236 * t589 * t498;
    let t73724 = t68421 * t68422 * t73722;
    (t73714, t73717, t73719, t73722, t73724)
}
