//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 874/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk874<F: Float>(t10524: F, t10629: F, t10632: F, t959: F, t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t291: F) -> (F, F, F, F) {
    let t10633 = t10629 * t10524 * t10632;
    let t10635 = 0.10254018858216406658e4 * t959 * t10633;
    let t10636 = 0.55403703703703703703e-1 * t10544;
    let t10647 = -t10636 - 0.23744444444444444444e-1 * t10556 + 0.11872222222222222222e-1 * t10558 - 0.35616666666666666666e-1 * t10560 + 0.17808333333333333333e-1 * t10562 - 0.19787037037037037037e-1 * t10566 + 0.71233333333333333332e-1 * t10569 - 0.35616666666666666666e-1 * t10530 - 0.10685e0 * t10572 + 0.10685e0 * t10538 - 0.17808333333333333333e-1 * t10575;
    let t10649 = 0.621814e-1 * t10647 * t291;
    (t10633, t10635, t10647, t10649)
}
