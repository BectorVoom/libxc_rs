//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 668/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk668<F: Float>(t24732: F, t3500: F, t483: F, t3068: F, t1244: F, t225: F, t460: F, t479: F, t2148: F, t3427: F, t2121: F, t24594: F, t23598: F, t50: F, t131: F, t467: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t24733 = t3500 * t24732;
    let t24739 = sigma2 * t483;
    let t24740 = t24739 * t3068;
    let t24741 = t1244 * t24740;
    let t24745 = t460 * t225;
    let t24746 = t24745 * t479;
    let t24771 = t3427 * t2148;
    let t24773 = 0.18277045187202515961e-2 * t2121 * t24771;
    let t24776 = t24594 * t225;
    let t24810 = t50 * t23598;
    let t24811 = t24810 * t131;
    let t24812 = t24811 * t467;
    (t24733, t24741, t24746, t24773, t24776, t24812)
}
