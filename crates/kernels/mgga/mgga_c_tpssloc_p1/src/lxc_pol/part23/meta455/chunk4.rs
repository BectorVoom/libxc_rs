//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1317/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1317<F: Float>(t1516: F, t16976: F, t20896: F, t20908: F, t40971: F, t4172: F, t46577: F, t5624: F, t5628: F, t58550: F, t67690: F, t67692: F, t67729: F, t67735: F, t68203: F, t75978: F, t76056: F, t820: F, t843: F, t847: F) -> F {
    let t76193 = F::new(35.0) / F::new(128.0) * t843 * t40971 * t820 * t76056 - F::new(5.0) / F::new(32.0) * t4172 * t20896 - t4172 * t20908 / F::new(192.0) - t843 * t847 * t820 * t75978 / F::new(768.0) + F::new(5.0) / F::new(128.0) * t16976 * t5624 - t16976 * t5628 / F::new(128.0) - t68203 * t1516 / F::new(192.0) - F::new(7.0) / F::new(96.0) * t67690 - F::new(7.0) / F::new(192.0) * t67692 - F::new(7.0) / F::new(96.0) * t67729 + F::new(7.0) / F::new(1152.0) * t67735 + F::new(595.0) / F::new(648.0) * t46577 - F::new(35.0) / F::new(36.0) * t58550;
    t76193
}
