//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 757/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk757<F: Float>(t14022: F, t14027: F, t15339: F, t458: F, t1430: F, t236: F, t14121: F, t14123: F, t14125: F, t21060: F, t495: F, t589: F) -> (F, F, F, F) {
    let t73708 = t15339 * t458 * t14022 * t14027;
    let t73712 = t236 * t1430;
    let t73714 = t21060 * t14121 * t14123 * t14125 * t73712;
    let t73717 = t236 * t589 * t495;
    (t73708, t73712, t73714, t73717)
}
