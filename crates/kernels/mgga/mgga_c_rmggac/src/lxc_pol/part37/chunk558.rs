//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 558/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk558<F: Float>(t15616: F, t656: F, t2145: F, t15297: F, t2265: F, t2415: F, t2010: F, t615: F, t698: F) -> (F, F, F, F, F, F) {
    let t15617 = t15616 * t656;
    let t15618 = t2145 * t15617;
    let t15619 = 0.34093327067806677161e-2 * t15618;
    let t15620 = 0.1276937996798935182e-4 * t15297;
    let t15621 = t2415 * t2265;
    let t15622 = t2010 * t15621;
    let t15623 = 0.36021158228745895953e-3 * t15622;
    let t15624 = t698 * t615;
    (t15617, t15619, t15620, t15621, t15623, t15624)
}
