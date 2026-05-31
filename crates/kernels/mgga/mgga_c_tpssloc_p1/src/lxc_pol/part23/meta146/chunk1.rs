//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 689/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk689<F: Float>(t383: F, t5914: F, t1058: F, t1610: F, t1630: F, t1632: F, t3186: F, t3200: F, t353: F, t384: F, t4669: F, t5903: F, t5929: F, t5933: F, t5937: F, t5939: F) -> (F, F) {
    let t5941 = t383 * t5914;
    let t5943 = F::cast_from(2.0_f64) * t1058 * t5933 + t1058 * t5937 + F::cast_from(2.0_f64) * t1610 * t1632 + F::cast_from(2.0_f64) * t1630 * t4669 + F::cast_from(2.0_f64) * t3186 * t5929 - t3200 * t5939 + t353 * t5941 + t384 * t5903;
    (t5941, t5943)
}
