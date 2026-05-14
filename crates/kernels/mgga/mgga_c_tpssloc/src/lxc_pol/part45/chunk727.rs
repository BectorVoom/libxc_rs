//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 727/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk727<F: Float>(t23664: F, t23720: F, t1055: F, t1065: F, t6815: F, t3174: F, t2780: F, t6690: F, t6689: F, t10170: F, t1052: F, t11010: F, t1956: F, t23579: F, t23582: F, t23589: F, t23595: F, t3026: F, t6680: F, t6687: F, t6700: F, t6816: F) -> (F,) {
    let t23721 = t23664 + t23720;
    let t23722 = t1055 * t23721;
    let t23724 = t6815 * t1065;
    let t23725 = t3174 * t23724;
    let t23728 = t6690 * t2780;
    let t23729 = t6689 * t23728;
    let t23732 = -t10170 * t1956 - 0.43864908449286038306e-1 * t6680 * t6700 - t11010 * t1956 + 0.18277045187202515961e-2 * t23579 + 0.54831135561607547884e-2 * t6687 * t23582 - 2.0 * t3026 * t6816 + 0.16449340668482264365e-1 * t6687 * t23589 + 0.36554090374405031923e-2 * t6687 * t23595 - t1052 * t23722 + 4.0 * t1052 * t23725 + 0.27415567780803773942e-2 * t6687 * t23729;
    (t23732,)
}
