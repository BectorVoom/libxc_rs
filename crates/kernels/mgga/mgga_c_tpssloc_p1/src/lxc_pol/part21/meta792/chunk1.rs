//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2753/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2753<F: Float>(t58021: F, t46278: F, t10126: F, t12895: F, t12915: F, t1484: F, t16662: F, t1877: F, t2522: F, t2523: F, t39483: F, t4255: F, t4314: F, t46213: F, t5527: F, t57996: F, t58005: F, t58008: F, t58009: F, t58020: F) -> (F, F, F) {
    let t58022 = F::cast_from(0.5848223622634646207e0_f64) * t58021;
    let t58023 = F::cast_from(0.32530743900905219526e-1_f64) * t46278;
    let t58024 = F::new(6.0) * t10126 * t4314 * t5527 + F::new(24.0) * t12895 * t4255 * t4314 + F::new(8.0) * t12915 * t1877 * t58009 + F::new(6.0) * t1484 * t2522 * t46213 + F::new(6.0) * t16662 * t2522 * t2523 + t39483 + t57996 + t58005 + t58008 + t58020 - t58022 + t58023;
    (t58022, t58023, t58024)
}
