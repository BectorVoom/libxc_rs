//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 754/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk754<F: Float>(t69924: F, t570: F, t68740: F, t1550: F, t14207: F, t2868: F, t2001: F, t305: F, t3141: F, t8580: F, t13866: F, t1986: F, t8614: F, t14374: F, t15231: F, t15344: F, t70123: F) -> (F, F, F, F, F, F, F, F) {
    let t75666 = 0.19863479950205658386e-4 * t69924;
    let t75674 = t68740 * t570;
    let t75675 = t1550 * t75674;
    let t75677 = t2868 * t14207;
    let t75678 = 0.79828278012425390427e-1 * t75677;
    let t75681 = t3141 * t2001 * t305 * t8580;
    let t75685 = t13866 * t1986 * t305 * t8614;
    let t75687 = t14374 * t15231;
    let t75689 = t70123 * t15344;
    (t75666, t75674, t75675, t75678, t75681, t75685, t75687, t75689)
}
