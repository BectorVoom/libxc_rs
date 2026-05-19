//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1199/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1199<F: Float>(t4028: F, t8327: F, t7458: F, t1774: F, t8326: F, t652: F, t1842: F, t31090: F, t22635: F, t1992: F, t6906: F, t7749: F) -> (F, F, F, F, F, F, F, F) {
    let t32673 = t4028 * t8327;
    let t32674 = F::new(2.0) * t32673;
    let t32675 = t7458 * t8327;
    let t32676 = F::new(2.0) * t32675;
    let t32677 = t1774 * t8326;
    let t32678 = t652 * t32677;
    let t32679 = F::new(2.0) * t32678;
    let t32693 = t31090 * t1842;
    let t32694 = t22635 * t32693;
    let t32696 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t32694;
    let t32697 = t6906 * t7749;
    (t32674, t32676, t32677, t32679, t32693, t32694, t32696, t32697)
}
