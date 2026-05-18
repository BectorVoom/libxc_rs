//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1222/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1222<F: Float>(t80970: F, t1336: F, t22759: F, t835: F, t12248: F, t6604: F, t22723: F, t268: F, t534: F, t22641: F, t3749: F, t1984: F, t80845: F) -> (F, F, F, F, F, F) {
    let t80971 = F::new(0.43737152435318756759e-3) * t80970;
    let t80997 = t1336 * t22759 * t835;
    let t81027 = t6604 * t12248;
    let t81046 = t22723 * t534 * t268;
    let t81064 = t22641 * t3749;
    let t81071 = t80845 * t1984;
    (t80971, t80997, t81027, t81046, t81064, t81071)
}
