//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 908/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk908<F: Float>(t10633: F, t959: F, t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F) -> (F, F) {
    let t10635 = F::cast_from(0.10254018858216406658e4_f64) * t959 * t10633;
    let t10636 = F::cast_from(0.55403703703703703703e-1_f64) * t10544;
    let t10647 = -t10636 - F::cast_from(0.23744444444444444444e-1_f64) * t10556 + F::cast_from(0.11872222222222222222e-1_f64) * t10558 - F::cast_from(0.35616666666666666666e-1_f64) * t10560 + F::cast_from(0.17808333333333333333e-1_f64) * t10562 - F::cast_from(0.19787037037037037037e-1_f64) * t10566 + F::cast_from(0.71233333333333333332e-1_f64) * t10569 - F::cast_from(0.35616666666666666666e-1_f64) * t10530 - F::cast_from(0.10685e0_f64) * t10572 + F::cast_from(0.10685e0_f64) * t10538 - F::cast_from(0.17808333333333333333e-1_f64) * t10575;
    (t10635, t10647)
}
