//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 992/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk992<F: Float>(t1527: F, t8362: F, t2718: F, t225: F, t258: F, t7510: F, t214: F, t1880: F, t1484: F, t30622: F, t23270: F, t22986: F, t30676: F, t6637: F, t6552: F, t232: F, t25261: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32803 = t8362 * t1527;
    let t32804 = t2718 * t32803;
    let t32808 = t7510 * t225 * t258;
    let t32809 = t214 * t32808;
    let t32811 = 0.16449340668482264365e-1 * t1880 * t32809;
    let t32814 = t30622 * t1484;
    let t32815 = t23270 * t32814;
    let t32817 = 0.3289868133696452873e-1 * t22986 * t32815;
    let t32818 = t30676 * t1484;
    let t32819 = t6637 * t32818;
    let t32821 = 0.3289868133696452873e-1 * t6552 * t32819;
    let t32822 = t25261 * t232;
    (t32804, t32808, t32809, t32811, t32814, t32815, t32817, t32818, t32819, t32821, t32822)
}
