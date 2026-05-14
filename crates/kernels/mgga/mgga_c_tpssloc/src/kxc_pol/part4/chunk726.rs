//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 726/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk726<F: Float>(t225: F, t5600: F, t2671: F, t5527: F, t5544: F, t824: F, t1504: F, t1506: F, t228: F, t230: F, t232: F, t819: F, t820: F, t5584: F, t2701: F, t847: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5601 = t5600 * t225;
    let t5605 = t2671 * t5527;
    let t5608 = t824 * t5544;
    let t5611 = 6.0 * t1504 * t1506 - 12.0 * t228 * t5605 + 3.0 * t228 * t5608 - t230 * t5601;
    let t5612 = t5611 * t232;
    let t5614 = t819 * t820 * t5612;
    let t5617 = t5584 * t232;
    let t5619 = t819 * t820 * t5617;
    let t5624 = t2701 * t820 * t5527;
    let t5628 = t847 * t820 * t5544;
    (t5601, t5605, t5608, t5611, t5612, t5614, t5617, t5619, t5624, t5628)
}
