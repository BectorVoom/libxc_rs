//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1488/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1488<F: Float>(t10524: F, t2932: F, t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F) -> (F, F, F) {
    let t10829 = t10524 * t2932;
    let t10832 = F::cast_from(0.53272592592592592592e-1_f64) * t10544;
    let t10843 = -t10832 - F::cast_from(0.2283111111111111111e-1_f64) * t10556 + F::cast_from(0.11415555555555555555e-1_f64) * t10558 - F::cast_from(0.34246666666666666665e-1_f64) * t10560 + F::cast_from(0.17123333333333333333e-1_f64) * t10562 - F::cast_from(0.19025925925925925925e-1_f64) * t10566 + F::cast_from(0.68493333333333333331e-1_f64) * t10569 - F::cast_from(0.34246666666666666665e-1_f64) * t10530 - F::new(0.10274e0) * t10572 + F::new(0.10274e0) * t10538 - F::cast_from(0.17123333333333333333e-1_f64) * t10575;
    (t10829, t10832, t10843)
}
