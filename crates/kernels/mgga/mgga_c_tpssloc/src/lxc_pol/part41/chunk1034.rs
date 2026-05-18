//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1034/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1034<F: Float>(t16586: F, t2658: F, t2523: F, t5527: F, t262: F, t5544: F, t1484: F, t868: F, t5660: F, t870: F, t12850: F, t12860: F, t16577: F, t16578: F, t16581: F, t16582: F, t16583: F, t2522: F, t4119: F, t4307: F, t4310: F, t4314: F, t776: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F) -> (F, F) {
    let t16587 = t2658 * t16586;
    let t16588 = F::new(12.0) * t16587;
    let t16589 = t2523 * t5527;
    let t16592 = t262 * t5544;
    let t16596 = t1484 * t868;
    let t16606 = t5660 * t870;
    let t16610 = F::new(6.0) * t16592 * t4314 * t776 - F::new(6.0) * t16596 * t2522 * t4307 + F::new(3.0) * t16606 * t2522 * t776 + F::new(3.0) * t2522 * t2523 * t5544 + F::new(6.0) * t2522 * t4119 * t4310 + F::new(12.0) * t16583 * t4314 + F::new(6.0) * t16589 * t4314 + t12850 - t12860 + t16577 + t16578 + t16581 + t16582 + t16588 - t9457 - t9469 + t9476 + t9484 - t9496;
    (t16588, t16610)
}
