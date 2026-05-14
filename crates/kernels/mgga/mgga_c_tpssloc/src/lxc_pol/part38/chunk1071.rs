//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1071/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1071<F: Float>(t1147: F, t1156: F, t14829: F, t1164: F, t3423: F, t4869: F, t11126: F, t1703: F, t1657: F, t3263: F, t3266: F, t11292: F, t1694: F, t3404: F, t1098: F, t4737: F) -> (F, F, F, F, F, F) {
    let t14831 = t1147 * t14829 * t1156;
    let t14833 = 0.5848223622634646207e0 * t1164 * t14831;
    let t14835 = 0.17315859105681463759e2 * t4869 * t3423;
    let t14837 = 0.5848223622634646207e0 * t11126 * t1703;
    let t14838 = t1657 * t3263;
    let t14840 = 2.0 * t14838 * t3266;
    let t14841 = t11292 * t1694;
    let t14842 = t14841 * t3404;
    let t14844 = 0.10389515463408878255e3 * t1164 * t14842;
    let t14845 = t4737 * t1098;
    (t14833, t14835, t14837, t14840, t14844, t14845)
}
