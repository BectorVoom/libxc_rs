//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1456/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1456<F: Float>(t1462: F, t9912: F, t9871: F, t2427: F, t4101: F, t9880: F, t2528: F, t4199: F, t2663: F, t4211: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F, t9894: F) -> (F, F, F, F, F, F, F) {
    let t13102 = F::new(4.0) * t9912 * t1462;
    let t13103 = F::cast_from(0.4883052614935078681e-3_f64) * t9871;
    let t13105 = F::new(8.0) * t2427 * t4101;
    let t13106 = F::cast_from(0.21687162600603479684e-1_f64) * t9880;
    let t13107 = t4199 * t2528;
    let t13108 = F::cast_from(0.17315859105681463759e2_f64) * t13107;
    let t13109 = t4211 * t2663;
    let t13110 = F::cast_from(0.24415263074675393405e-3_f64) * t13109;
    let t13111 = t13102 + t13103 + t9793 + t9797 - t9876 + t13105 - t9820 - t9824 + t13106 - t9884 + t9887 + t9890 - t13108 - t9894 + t13110;
    (t13102, t13103, t13105, t13106, t13108, t13110, t13111)
}
