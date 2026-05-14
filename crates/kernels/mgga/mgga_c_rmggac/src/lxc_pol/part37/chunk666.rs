//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 666/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk666<F: Float>(t446: F, t511: F, t558: F, t14117: F, t68448: F, t68455: F, t9205: F, t14022: F, t14027: F, t15339: F, t458: F, t1430: F, t236: F, t14121: F, t14123: F, t14125: F, t21060: F) -> (F, F, F, F, F, F) {
    let t73699 = t511 * t558 * t446;
    let t73701 = t68448 * t14117 * t73699;
    let t73704 = t68455 * t14117 * t9205;
    let t73708 = t15339 * t458 * t14022 * t14027;
    let t73712 = t236 * t1430;
    let t73714 = t21060 * t14121 * t14123 * t14125 * t73712;
    (t73699, t73701, t73704, t73708, t73712, t73714)
}
