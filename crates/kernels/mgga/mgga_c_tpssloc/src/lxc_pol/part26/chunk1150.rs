//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1150/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1150<F: Float>(t81766: F, t849: F, t23132: F, t2617: F, t23133: F, t2707: F, t6621: F, t9997: F, t23127: F, t2703: F, t81724: F, t81728: F, t81731: F, t81736: F, t81738: F, t81743: F, t81746: F, t81750: F, t81752: F, t81754: F, t81756: F, t81758: F, t81760: F, t81764: F) -> (F,) {
    let t81767 = t81766 * t849;
    let t81769 = t2617 * t23132;
    let t81770 = t81769 * t849;
    let t81772 = t23133 * t2707;
    let t81774 = t6621 * t9997;
    let t81776 = t23127 * t2703;
    let t81778 = t81724 / 256.0 - 0.72670960969452703536e-2 * t81728 + 0.12111826828242117256e-2 * t81731 - t81736 - 0.60559134141210586281e-3 * t81738 + t81743 + 0.36335480484726351768e-2 * t81746 - 7.0 / 96.0 * t81750 + t81752 / 128.0 + t81754 / 128.0 - t81756 / 64.0 - t81758 / 512.0 - t81760 / 128.0 - 119.0 / 576.0 * t81764 - t81767 / 128.0 + 7.0 / 96.0 * t81770 + 7.0 / 192.0 * t81772 - t81774 / 384.0 + 5.0 / 128.0 * t81776;
    (t81778,)
}
