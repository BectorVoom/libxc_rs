//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1175/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1175<F: Float>(t26149: F, t8450: F, t33133: F, t7000: F, t33160: F, t6876: F, t26502: F, t3701: F, t1983: F, t2019: F, t26142: F, t6517: F, t25994: F, t19456: F, t8323: F, t31069: F, t4028: F) -> (F, F, F, F, F, F, F, F) {
    let t120003 = t8450 * t26149;
    let t120005 = t33133 * t7000;
    let t120008 = 3.0 * t6876 * t33160;
    let t120016 = t3701 * t26502;
    let t120019 = 2.0 * t1983 * t2019 * t120016;
    let t120020 = t6517 * t26142;
    let t120022 = t6517 * t25994;
    let t120027 = t19456 * t8323;
    let t120029 = t4028 * t31069;
    (t120003, t120005, t120008, t120019, t120020, t120022, t120027, t120029)
}
