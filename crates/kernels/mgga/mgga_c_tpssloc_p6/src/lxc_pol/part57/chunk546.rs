//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 546/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk546<F: Float>(t225: F, t6688: F, t362: F, t381: F, t334: F, t371: F, t38: F, t131: F, t350: F, t1009: F, t344: F, t1014: F) -> (F, F, F, F, F, F) {
    let t6784 = t6688 * t225;
    let t6785 = t362 * t381;
    let t6793 = t371 * t334;
    let t6794 = F::new(1.0) / t6793;
    let t6795 = t38 * t6794;
    let t6796 = t6795 * t131;
    let t6797 = t6796 * t350;
    let t6798 = t344 * t1009;
    let t6799 = t6798 * t1014;
    (t6784, t6785, t6795, t6796, t6797, t6799)
}
