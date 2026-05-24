//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1077/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1077<F: Float>(t71447: F, t73353: F, t77559: F, t77560: F, t77562: F, t77567: F, t77573: F, t77575: F, t77578: F, t77581: F, t77584: F, t77585: F, t77586: F, t77587: F, t77589: F, t77590: F, t77591: F) -> F {
    let t80248 = -t77559 + t77560 + t71447 + t77562 + t77567 + t77573 + t77575 + t77578 - t77581 - t77584 - t77585 - t77586 + t73353 - t77587 + t77589 - t77590 - t77591;
    t80248
}
