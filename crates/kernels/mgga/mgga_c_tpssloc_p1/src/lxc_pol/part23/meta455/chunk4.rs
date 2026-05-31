//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1317/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1317<F: Float>(t1516: F, t16976: F, t20896: F, t20908: F, t40971: F, t4172: F, t46577: F, t5624: F, t5628: F, t58550: F, t67690: F, t67692: F, t67729: F, t67735: F, t68203: F, t75978: F, t76056: F, t820: F, t843: F, t847: F) -> F {
    let t76193 = F::cast_from(35.0_f64) / F::cast_from(128.0_f64) * t843 * t40971 * t820 * t76056 - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t4172 * t20896 - t4172 * t20908 / F::cast_from(192.0_f64) - t843 * t847 * t820 * t75978 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t16976 * t5624 - t16976 * t5628 / F::cast_from(128.0_f64) - t68203 * t1516 / F::cast_from(192.0_f64) - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t67690 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t67692 - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t67729 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t67735 + F::cast_from(595.0_f64) / F::cast_from(648.0_f64) * t46577 - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t58550;
    t76193
}
