//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 921/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk921<F: Float>(t75733: F, t530: F, t71582: F, t73255: F, t73411: F, t77795: F, t77796: F, t77797: F, t77803: F, t77804: F, t77807: F, t77810: F, t77812: F, t77820: F, t77823: F, t77824: F, t77825: F) -> (F,) {
    let t80307 = 0.29085809927086856922e-4 * t75733;
    let t80308 = -t77795 + t77796 - t77797 + t71582 + t77803 - t77804 - 0.2363e1 * t530 * t73255 + t77807 + t77810 + t77812 + t73411 - t77820 + t80307 + t77823 + t77824 - t77825;
    (t80308,)
}
