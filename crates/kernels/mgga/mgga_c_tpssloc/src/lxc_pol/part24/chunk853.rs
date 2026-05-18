//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 853/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk853<F: Float>(t763: F, t9716: F, t177: F, t2508: F, t2512: F, t9490: F, t761: F, t9450: F, t9457: F, t9463: F, t9469: F, t9476: F, t9484: F, t9496: F, t9684: F, t9715: F) -> (F, F, F, F, F) {
    let t9717 = t9716 * t763;
    let t9718 = F::new(0.17544670867903938621e1) * t9717;
    let t9720 = F::new(1.0) / t2508 / t177;
    let t9722 = t9720 * t9490 * t2512;
    let t9724 = F::new(0.10389515463408878255e3) * t761 * t9722;
    let t9725 = t9450 - t9457 + t9463 - t9469 + t9476 + t9484 - t9496 + t9684 - t9715 - t9718 + t9724;
    (t9718, t9720, t9722, t9724, t9725)
}
