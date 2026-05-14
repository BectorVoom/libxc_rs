//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 703/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk703<F: Float>(t13858: F, t2412: F, t15220: F, t2191: F, t1986: F, t675: F, t8958: F, t13862: F, t1654: F, t3133: F, t14011: F, t1603: F, t3120: F, t14150: F, t290: F, t39116: F, t70127: F) -> (F, F, F, F, F, F) {
    let t74579 = t2412 * t13858;
    let t74581 = t2191 * t15220;
    let t74584 = t675 * t1986 * t8958;
    let t74587 = t3133 * t13862 * t1654;
    let t74590 = t3120 * t14011 * t1603;
    let t74594 = t70127 * t39116 * t290 * t14150;
    (t74579, t74581, t74584, t74587, t74590, t74594)
}
