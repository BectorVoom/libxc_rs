//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2745/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2745<F: Float>(t46134: F, t46137: F, t4303: F, t776: F, t2517: F, t5520: F, t40667: F, t40673: F, t40680: F, t2522: F, t39309: F, t39312: F, t39316: F, t39320: F, t40679: F, t4307: F) -> (F, F, F, F, F, F, F) {
    let t57891 = F::cast_from(0.96319466275353142155e0_f64) * t46134;
    let t57892 = F::cast_from(0.43374325201206959368e-1_f64) * t46137;
    let t57893 = t776 * t4303;
    let t57897 = t5520 * t2517;
    let t57898 = F::cast_from(0.10389515463408878255e3_f64) * t40667;
    let t57899 = F::new(2.0) * t40673;
    let t57900 = F::cast_from(0.24415263074675393405e-3_f64) * t40680;
    let t57901 = -F::new(12.0) * t2522 * t4307 * t57893 - t39309 + t39312 + t39316 + t39320 - t40679 + t57891 + t57892 + t57897 - t57898 + t57899 + t57900;
    (t57891, t57892, t57897, t57898, t57899, t57900, t57901)
}
