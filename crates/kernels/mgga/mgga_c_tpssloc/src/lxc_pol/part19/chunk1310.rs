//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1310/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1310<F: Float>(t43748: F, t43750: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t43819: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43811: F, t43816: F, t43823: F, t43828: F) -> (F, F) {
    let t44342 = -0.31659259259259259258e-1 * t43748 - 0.26382716049382716049e-1 * t43750 + 0.47488888888888888888e-1 * t43780 + 0.94977777777777777776e-1 * t43782 + 0.94977777777777777776e-1 * t43784 - 0.14246666666666666667e0 * t43786 - 0.23744444444444444444e-1 * t43788 + 0.23744444444444444444e0 * t43794 - 0.42739999999999999999e0 * t43798 + 0.4274e0 * t43802 + 0.17808333333333333333e-1 * t43806;
    let t44348 = 0.18467901234567901234e0 * t43819;
    let t44355 = -0.52765432098765432099e-1 * t43811 + 0.47488888888888888888e-1 * t43727 - 0.14246666666666666667e0 * t43729 + 0.11872222222222222222e0 * t43734 - 0.73871604938271604937e-1 * t43816 + t44348 - 0.42739999999999999999e0 * t43737 - 0.35616666666666666666e-1 * t43823 - 0.47488888888888888888e-1 * t43740 + 0.6411e0 * t43743 + 0.10685e0 * t43828 + 0.14246666666666666667e0 * t43746;
    (t44342, t44355)
}
