//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1235/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1235<F: Float>(t26502: F, t3701: F, t1983: F, t2019: F, t26142: F, t6517: F, t25994: F, t19456: F, t8323: F, t31069: F, t4028: F, t33085: F, t6525: F) -> (F, F, F, F, F, F) {
    let t120016 = t3701 * t26502;
    let t120019 = F::cast_from(2.0_f64) * t1983 * t2019 * t120016;
    let t120020 = t6517 * t26142;
    let t120022 = t6517 * t25994;
    let t120027 = t19456 * t8323;
    let t120029 = t4028 * t31069;
    let t120040 = t33085 * t6525;
    (t120019, t120020, t120022, t120027, t120029, t120040)
}
