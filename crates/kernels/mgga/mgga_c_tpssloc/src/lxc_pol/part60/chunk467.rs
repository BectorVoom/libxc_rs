//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 467/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk467<F: Float>(t5526: F, t5668: F, t2770: F, t5392: F, t2768: F, t123: F, t2775: F, t882: F, t5398: F, t883: F, t2765: F, t4335: F) -> (F, F, F, F, F, F, F, F) {
    let t5669 = t5526 + t5668;
    let t5677 = t2770 * t5392;
    let t5678 = t2768 * t5677;
    let t5679 = t123 * t5678;
    let t5681 = t2775 * t5392;
    let t5682 = t882 * t5681;
    let t5683 = t123 * t5682;
    let t5685 = t883 * t5398;
    let t5686 = t882 * t5685;
    let t5687 = t123 * t5686;
    let t5689 = t2765 + F::new(0.11872222222222222222e-1) * t4335 - F::new(0.11872222222222222222e-1) * t5679 + F::new(0.35616666666666666666e-1) * t5683 - F::new(0.17808333333333333333e-1) * t5687;
    (t5669, t5677, t5679, t5681, t5683, t5685, t5687, t5689)
}
