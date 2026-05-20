//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2341/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2341<F: Float>(t20944: F, t41011: F, t119: F, t13365: F, t1516: F, t16976: F, t20943: F, t210: F, t2571: F, t41084: F, t41161: F, t4119: F, t4158: F, t4261: F, t46887: F, t46912: F, t46929: F, t5544: F, t5567: F, t5624: F, t58744: F, t58834: F, t67282: F, t776: F, t787: F, t820: F, t843: F, t847: F, t9559: F) -> F {
    let t67937 = t41011 * t20944;
    let t67957 = -t843 * t847 * t820 * t67282 / F::new(768.0) + F::new(5.0) / F::new(256.0) * t13365 * t5624 - t58834 * t1516 / F::new(256.0) - t16976 * t4261 / F::new(256.0) + t46887 + F::new(7.0) / F::new(12.0) * t67937 - t787 * t210 * t119 * t67282 / F::new(48.0) + t46912 + F::new(35.0) / F::new(24.0) * t58744 + F::new(455.0) / F::new(648.0) * t41084 - t46929 + F::new(5.0) / F::new(4.0) * t41161 * t210 * t20943 * t776 - F::new(3.0) / F::new(4.0) * t9559 * t210 * t5567 * t4119 + F::new(3.0) / F::new(16.0) * t2571 * t210 * t4158 * t5544;
    t67957
}
