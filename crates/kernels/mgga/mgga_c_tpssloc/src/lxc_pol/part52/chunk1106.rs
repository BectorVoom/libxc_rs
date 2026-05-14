//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1106/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1106<F: Float>(t5: F, t7973: F, t8301: F, t2240: F, t31860: F, t31864: F, t33115: F, t33564: F, t33568: F, t33572: F, t33669: F, t8515: F, t8663: F, t112: F, t7266: F, t7468: F, t1458: F, t2113: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t33676 = t8301 * t7973;
    let t33677 = t2240 * t33676;
    let t33685 = piecewise3(t8, 0.0, 5.0 / 144.0 * t33669 * t8515 - 5.0 / 24.0 * t31860 * t33564 - 5.0 / 36.0 * t31864 * t33568 + 5.0 / 144.0 * t33677 * t8515 + 5.0 / 72.0 * t8663 * t33572 + 5.0 / 144.0 * t8663 * t33115);
    let t33686 = t33685 * t112;
    let t33688 = t7266 * t7468;
    let t33690 = t2113 * t1458;
    (t33676, t33677, t33685, t33686, t33688, t33690)
}
