//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 667/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk667<F: Float>(t22870: F, t553: F, t1338: F, t6955: F, t1352: F, t3851: F, t6987: F, t3856: F, t1372: F, t552: F, t1307: F, t6637: F, t6888: F, t3719: F, t6968: F, t117: F, t547: F, t67: F) -> (F, F, F, F, F, F, F) {
    let t22871 = t553 * t22870;
    let t22873 = t1338 * t6955;
    let t22874 = t22873 * t1352;
    let t22877 = t6987 * t3851;
    let t22879 = t6987 * t3856;
    let t22881 = t552 * t1372;
    let t22882 = t22881 * t1307;
    let t22883 = t6637 * t22882;
    let t22884 = t6888 * t22883;
    let t22886 = t6968 * t3719;
    let t22887 = t6637 * t22886;
    let t22888 = t6888 * t22887;
    let t22891 = t547 * t67 * t117;
    (t22871, t22874, t22877, t22879, t22884, t22888, t22891)
}
