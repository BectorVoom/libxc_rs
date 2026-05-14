//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1133/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1133<F: Float>(t22674: F, t22934: F, t6897: F, t1307: F, t1377: F, t22633: F, t22635: F, t3911: F, t22935: F, t6883: F, t22667: F, t1987: F, t81144: F, t9537: F, t12438: F, t12444: F, t22653: F, t22656: F, t22905: F, t3758: F, t3882: F, t3889: F, t539: F, t568: F, t6958: F, t6993: F, t81011: F) -> (F,) {
    let t81379 = t6897 * t22674 * t22934;
    let t81386 = t22633 * t22635 * t1377 * t3911 * t1307;
    let t81393 = t6883 * t22935;
    let t81395 = t6883 * t22667;
    let t81398 = t81144 * t9537 * t1987;
    let t81399 = 0.13707783890401886971e-2 * t81398;
    let t81404 = -0.24674011002723396548e-1 * t81379 - 6.0 * t12444 * t6993 + 0.49348022005446793095e-1 * t81386 + t539 * t81011 * t568 - t6958 * t12438 + 6.0 * t22656 * t3889 - 0.11514538467937585055e0 * t81393 + 0.11514538467937585055e0 * t81395 - t81399 - 3.0 * t3882 * t22905 + 12.0 * t3758 * t22653;
    (t81404,)
}
