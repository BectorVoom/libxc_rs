//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1183/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1183<F: Float>(t1147: F, t1156: F, t14829: F, t1164: F, t3423: F, t4869: F, t11126: F, t1703: F, t1657: F, t3263: F, t3266: F, t11292: F, t1694: F) -> (F, F, F, F, F) {
    let t14831 = t1147 * t14829 * t1156;
    let t14833 = F::cast_from(0.5848223622634646207e0_f64) * t1164 * t14831;
    let t14835 = F::cast_from(0.17315859105681463759e2_f64) * t4869 * t3423;
    let t14837 = F::cast_from(0.5848223622634646207e0_f64) * t11126 * t1703;
    let t14838 = t1657 * t3263;
    let t14840 = F::new(2.0) * t14838 * t3266;
    let t14841 = t11292 * t1694;
    (t14833, t14835, t14837, t14840, t14841)
}
