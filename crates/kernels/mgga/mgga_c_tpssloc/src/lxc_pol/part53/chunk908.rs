//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 908/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk908<F: Float>(t116498: F, t123414: F, t123418: F, t123745: F, t123752: F, t16596: F, t1877: F, t24339: F, t24344: F, t2522: F, t25365: F, t25374: F, t26739: F, t32047: F, t4119: F, t4255: F, t4303: F, t4314: F, t7114: F, t7844: F, t8748: F) -> (F,) {
    let t123835 = -6.0 * t116498 * t1877 * t25374 - 6.0 * t123414 * t2522 * t7114 + 4.0 * t123418 * t1877 * t24344 - 6.0 * t123745 * t2522 * t7114 + 4.0 * t123752 * t1877 * t24344 + 6.0 * t16596 * t2522 * t32047 - 2.0 * t1877 * t24339 * t7844 - 2.0 * t1877 * t26739 * t7114 + 2.0 * t1877 * t32047 * t4303 + 6.0 * t2522 * t25365 * t32047 - 3.0 * t2522 * t4119 * t8748 - 6.0 * t4255 * t4314 * t8748;
    (t123835,)
}
