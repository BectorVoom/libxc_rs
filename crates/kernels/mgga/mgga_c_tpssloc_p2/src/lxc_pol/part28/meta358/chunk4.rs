//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1340/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1340<F: Float>(t10126: F, t13095: F, t13096: F, t13098: F, t13102: F, t13103: F, t13105: F, t13106: F, t13108: F, t1484: F, t2522: F, t2523: F, t4119: F, t9789: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F) -> F {
    let t13483 = F::new(3.0) * t10126 * t1484 * t2522 + F::new(6.0) * t2522 * t2523 * t4119 + t13095 + t13096 + t13098 + t13102 + t13103 + t13105 + t13106 - t13108 - t9789 + t9793 + t9797 - t9820 - t9824 - t9876 - t9884 + t9887 + t9890;
    t13483
}
