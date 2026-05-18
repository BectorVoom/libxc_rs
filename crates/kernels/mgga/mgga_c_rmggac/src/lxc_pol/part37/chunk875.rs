//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 875/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk875<F: Float>(t13866: F, t1986: F, t305: F, t8614: F, t14374: F, t15231: F, t15344: F, t70123: F, t13862: F, t1616: F, t3133: F, t14011: F, t1654: F, t3120: F) -> (F, F, F, F, F) {
    let t75685 = t13866 * t1986 * t305 * t8614;
    let t75687 = t14374 * t15231;
    let t75689 = t70123 * t15344;
    let t75692 = t3133 * t13862 * t1616;
    let t75695 = t3120 * t14011 * t1654;
    (t75685, t75687, t75689, t75692, t75695)
}
