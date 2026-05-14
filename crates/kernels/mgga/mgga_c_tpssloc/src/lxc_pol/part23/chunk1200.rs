//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1200/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1200<F: Float>(t1516: F, t16976: F, t20896: F, t20908: F, t40971: F, t4172: F, t46577: F, t5624: F, t5628: F, t58550: F, t67690: F, t67692: F, t67729: F, t67735: F, t68203: F, t75978: F, t76056: F, t820: F, t843: F, t847: F) -> (F,) {
    let t76193 = 35.0 / 128.0 * t843 * t40971 * t820 * t76056 - 5.0 / 32.0 * t4172 * t20896 - t4172 * t20908 / 192.0 - t843 * t847 * t820 * t75978 / 768.0 + 5.0 / 128.0 * t16976 * t5624 - t16976 * t5628 / 128.0 - t68203 * t1516 / 192.0 - 7.0 / 96.0 * t67690 - 7.0 / 192.0 * t67692 - 7.0 / 96.0 * t67729 + 7.0 / 1152.0 * t67735 + 595.0 / 648.0 * t46577 - 35.0 / 36.0 * t58550;
    (t76193,)
}
