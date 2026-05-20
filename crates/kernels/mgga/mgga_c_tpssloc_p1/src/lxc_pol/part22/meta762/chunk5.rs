//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2569/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2569<F: Float>(t50846: F, t51271: F, t71146: F, t71150: F, t71152: F, t71154: F, t71156: F, t71160: F, t71166: F, t71170: F, t71174: F, t71179: F) -> F {
    let t71941 = -F::cast_from(0.19128703703703703704e0_f64) * t71146 + F::new(0.516475e0) * t71150 - F::new(0.103295e1) * t71152 - F::cast_from(0.17215833333333333333e0_f64) * t71154 + F::cast_from(0.68863333333333333333e0_f64) * t71156 - F::cast_from(0.92617777777777777779e0_f64) * t50846 + t51271 + F::cast_from(0.17215833333333333334e1_f64) * t71160 - F::cast_from(0.15302962962962962963e1_f64) * t71166 + F::new(0.929655e1) * t71170 + F::new(0.123954e2) * t71174 + F::new(0.103295e1) * t71179;
    t71941
}
