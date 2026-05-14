//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 788/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk788<F: Float>(t761: F, t9722: F, t9450: F, t9457: F, t9463: F, t9469: F, t9476: F, t9484: F, t9496: F, t9684: F, t9715: F, t9718: F, t2517: F, t718: F, t2475: F, t723: F) -> (F, F, F, F) {
    let t9724 = 0.10389515463408878255e3 * t761 * t9722;
    let t9725 = t9450 - t9457 + t9463 - t9469 + t9476 + t9484 - t9496 + t9684 - t9715 - t9718 + t9724;
    let t9726 = t718 * t2517;
    let t9727 = 3.0 * t9726;
    let t9729 = 1.0 / t2475 / t723;
    (t9724, t9725, t9727, t9729)
}
