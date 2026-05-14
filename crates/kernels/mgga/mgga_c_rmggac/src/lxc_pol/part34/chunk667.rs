//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 667/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk667<F: Float>(t14012: F, t73794: F, t14015: F, t3154: F, t38355: F, t13858: F, t8571: F, t15363: F, t69568: F, t14236: F, t14237: F, t1528: F, t2067: F, t26: F, t15388: F, t68538: F) -> (F, F, F, F, F, F, F) {
    let t73799 = t73794 * t14012;
    let t73801 = t73794 * t14015;
    let t73803 = t38355 * t3154;
    let t73805 = t8571 * t13858;
    let t73807 = t69568 * t15363;
    let t73812 = t14236 * t14237 * t2067 * t26 * t1528;
    let t73814 = t68538 * t15388;
    (t73799, t73801, t73803, t73805, t73807, t73812, t73814)
}
