//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2526/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2526<F: Float>(t43816: F, t44348: F, t51565: F, t51574: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t71166: F, t71170: F, t71174: F, t71179: F, t71183: F, t71187: F, t71191: F, t71195: F, t71199: F, t71203: F, t71206: F) -> F {
    let t71214 = -F::cast_from(0.52765432098765432099e-1_f64) * t71166 + F::cast_from(0.32055e0_f64) * t71170 + F::cast_from(0.4274e0_f64) * t71174 + F::cast_from(0.35616666666666666666e-1_f64) * t71179 - F::cast_from(0.35616666666666666666e-1_f64) * t71183 - F::cast_from(0.35616666666666666666e-1_f64) * t71187 + F::cast_from(0.10685e0_f64) * t71191 - F::cast_from(0.2137e0_f64) * t71195 - F::cast_from(0.42739999999999999999e0_f64) * t71199 + F::cast_from(0.10685e0_f64) * t71203 + F::cast_from(0.32055e0_f64) * t71206 - t51565 + t51574 + t44348 - F::cast_from(0.18467901234567901234e-1_f64) * t43816 + F::cast_from(0.47488888888888888888e-1_f64) * t63361 + F::cast_from(0.23744444444444444444e-1_f64) * t63382 + F::cast_from(0.71233333333333333332e-1_f64) * t63384 - F::cast_from(0.71233333333333333332e-1_f64) * t63398 - F::cast_from(0.10685e0_f64) * t63400;
    t71214
}
