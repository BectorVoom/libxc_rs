//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 868/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk868<F: Float>(t112820: F, t112773: F, t112782: F, t112784: F, t112788: F, t112795: F, t112798: F, t112807: F, t112811: F, t112814: F, t114714: F, t114720: F, t114724: F, t112834: F, t112840: F, t112846: F) -> (F, F, F, F) {
    let t114725 = 7.0 / 144.0 * t112820;
    let t114726 = t112773 / 96.0 + t114714 + 0.67826230238155856632e-1 * t112782 + 0.13565246047631171327e0 * t112784 - 0.96894614625936938046e-2 * t112788 + t112795 / 384.0 - t112798 / 384.0 + t114720 - t112807 / 768.0 - t112811 / 768.0 + 0.32298204875312312682e-2 * t112814 + t114724 + t114725;
    let t114732 = 0.42167100809435519335e-2 * t112834;
    let t114734 = 0.13457585364713463618e-3 * t112840;
    let t114736 = 7.0 / 576.0 * t112846;
    (t114726, t114732, t114734, t114736)
}
