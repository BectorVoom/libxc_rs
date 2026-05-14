//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 890/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk890<F: Float>(t10620: F, t300: F, t2897: F, t961: F, t2940: F, t2948: F, t2928: F, t941: F, t2931: F, t323: F, t10524: F, t959: F, t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F) -> (F, F, F, F, F, F, F) {
    let t10622 = 0.19751673498613801407e-1 * t300 * t10620;
    let t10623 = t300 * t2897;
    let t10625 = 0.17544670867903938621e1 * t10623 * t961;
    let t10627 = 0.17544670867903938621e1 * t2940 * t2948;
    let t10629 = 1.0 / t2928 / t941;
    let t10632 = 1.0 / t2931 / t323;
    let t10633 = t10629 * t10524 * t10632;
    let t10635 = 0.10254018858216406658e4 * t959 * t10633;
    let t10636 = 0.55403703703703703703e-1 * t10544;
    let t10647 = -t10636 - 0.23744444444444444444e-1 * t10556 + 0.11872222222222222222e-1 * t10558 - 0.35616666666666666666e-1 * t10560 + 0.17808333333333333333e-1 * t10562 - 0.19787037037037037037e-1 * t10566 + 0.71233333333333333332e-1 * t10569 - 0.35616666666666666666e-1 * t10530 - 0.10685e0 * t10572 + 0.10685e0 * t10538 - 0.17808333333333333333e-1 * t10575;
    (t10622, t10625, t10627, t10629, t10632, t10635, t10647)
}
