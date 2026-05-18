//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 820/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk820<F: Float>(t36634: F, t656: F, t8950: F, t34944: F, t8979: F, t14125: F, t68440: F, t8835: F, t8842: F, t13962: F, t3056: F, t8486: F) -> (F, F, F, F, F) {
    let t74765 = t36634 * t656 * t8950;
    let t74768 = t34944 * t656 * t8979;
    let t74772 = t68440 * t14125 * t8835;
    let t74775 = t68440 * t14125 * t8842;
    let t74779 = t3056 * t13962 * t8486;
    (t74765, t74768, t74772, t74775, t74779)
}
