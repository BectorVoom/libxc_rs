//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 933/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk933<F: Float>(t1911: F, t857: F, t776: F, t23270: F, t22986: F, t6662: F, t2718: F, t2717: F) -> (F, F, F, F, F, F) {
    let t30622 = t857 * t1911;
    let t30623 = t30622 * t776;
    let t30624 = t23270 * t30623;
    let t30626 = 0.3289868133696452873e-1 * t22986 * t30624;
    let t30629 = t1911 * t6662;
    let t30630 = t2718 * t30629;
    let t30633 = t2717 * t1911;
    (t30622, t30623, t30624, t30626, t30630, t30633)
}
