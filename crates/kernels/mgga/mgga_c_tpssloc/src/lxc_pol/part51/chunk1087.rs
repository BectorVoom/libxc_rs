//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1087/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1087<F: Float>(t3701: F, t7216: F, t4028: F, t8327: F, t7458: F, t1774: F, t8326: F, t652: F, t1842: F, t31090: F, t22635: F, t1992: F, t6906: F, t7749: F, t6889: F, t1985: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32193 = t3701 * t7216;
    let t32673 = t4028 * t8327;
    let t32674 = 2.0 * t32673;
    let t32675 = t7458 * t8327;
    let t32676 = 2.0 * t32675;
    let t32677 = t1774 * t8326;
    let t32678 = t652 * t32677;
    let t32679 = 2.0 * t32678;
    let t32693 = t31090 * t1842;
    let t32694 = t22635 * t32693;
    let t32696 = 0.3289868133696452873e-1 * t1992 * t32694;
    let t32697 = t6906 * t7749;
    let t32698 = t6889 * t32697;
    let t32700 = 0.16449340668482264365e-1 * t1985 * t32698;
    (t32193, t32674, t32676, t32677, t32679, t32693, t32694, t32696, t32697, t32698, t32700)
}
