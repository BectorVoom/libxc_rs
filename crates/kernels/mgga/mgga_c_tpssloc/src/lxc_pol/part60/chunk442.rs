//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 442/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk442<F: Float>(t5694: F, t913: F, t2792: F, t1547: F, t2798: F, t2802: F, t4335: F, t5679: F, t5683: F, t5687: F, t894: F, t2815: F, t901: F, t2826: F, t5677: F, t136: F) -> (F, F, F, F, F, F) {
    let t5695 = t5694 * t913;
    let t5697 = 2.0 * t2792 * t5695;
    let t5698 = t1547 * t1547;
    let t5699 = t2798 * t5698;
    let t5705 = t2802 + 2.0 / 9.0 * t4335 - 2.0 / 9.0 * t5679 + 2.0 / 3.0 * t5683 - t5687 / 3.0;
    let t5706 = t894 * t5705;
    let t5712 = t2815 * t5698;
    let t5714 = t901 * t5705;
    let t5717 = t2826 * t5677;
    let t5718 = t136 * t5717;
    (t5697, t5699, t5706, t5712, t5714, t5718)
}
