//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2484/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2484<F: Float>(t14725: F, t9288: F, t136: F, t3297: F, t14748: F, t2250: F, t1113: F, t14735: F, t2244: F, t4728: F, t9258: F, t43768: F, t43770: F, t43777: F, t50846: F, t50848: F, t50851: F, t50854: F, t50859: F) -> (F, F, F, F, F, F, F, F, F) {
    let t50861 = t14725 * t9288;
    let t50863 = t136 * t3297 * t50861;
    let t50865 = t14748 * t2250;
    let t50867 = t136 * t1113 * t50865;
    let t50869 = t14735 * t2244;
    let t50871 = t136 * t1113 * t50869;
    let t50873 = t4728 * t9258;
    let t50875 = t136 * t1113 * t50873;
    let t50877 = -F::cast_from(0.24528888888888888889e0_f64) * t50846 - F::cast_from(0.16557e0_f64) * t50848 + F::cast_from(0.82785e-1_f64) * t50851 + t50854 + F::cast_from(0.55190000000000000001e-1_f64) * t43768 - F::cast_from(0.33114e0_f64) * t43770 + t43777 - F::cast_from(0.27595e-1_f64) * t50859 - F::cast_from(0.99342e0_f64) * t50863 + F::cast_from(0.49671e0_f64) * t50867 + F::cast_from(0.149013e1_f64) * t50871 + F::cast_from(0.16557e0_f64) * t50875;
    (t50861, t50863, t50865, t50867, t50869, t50871, t50873, t50875, t50877)
}
