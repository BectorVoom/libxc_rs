//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2520/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2520<F: Float>(t50845: F, t50877: F, t50902: F, t50942: F, t50974: F, t50996: F, t51032: F, t51104: F, t1147: F, t1156: F, t1164: F, t14831: F, t3411: F) -> (F, F, F) {
    let t51107 = t50845 + t50877 + t50902 + t50942 + t50974 + t50996 + t51032 + t51104;
    let t51111 = F::cast_from(0.5848223622634646207e0_f64) * t1164 * t1147 * t51107 * t1156;
    let t51113 = F::cast_from(0.17544670867903938621e1_f64) * t3411 * t14831;
    (t51107, t51111, t51113)
}
