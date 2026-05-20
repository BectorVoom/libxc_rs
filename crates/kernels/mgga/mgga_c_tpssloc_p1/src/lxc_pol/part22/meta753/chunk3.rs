//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2532/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2532<F: Float>(t43816: F, t43942: F, t50919: F, t51707: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t71166: F, t71170: F, t71174: F, t71179: F, t71183: F, t71187: F, t71191: F, t71195: F, t71199: F, t71203: F, t71206: F) -> F {
    let t71308 = -F::cast_from(0.27469135802469135803e-1_f64) * t71166 + F::new(0.166875e0) * t71170 + F::new(0.2225e0) * t71174 + F::cast_from(0.18541666666666666667e-1_f64) * t71179 - F::cast_from(0.18541666666666666666e-1_f64) * t71183 - F::cast_from(0.18541666666666666666e-1_f64) * t71187 + F::cast_from(0.55625000000000000001e-1_f64) * t71191 - F::new(0.11125e0) * t71195 - F::cast_from(0.22249999999999999999e0_f64) * t71199 + F::cast_from(0.55625000000000000001e-1_f64) * t71203 + F::new(0.166875e0) * t71206 - F::cast_from(0.82407407407407407407e-2_f64) * t50919 + t51707 + t43942 - F::cast_from(0.96141975308641975307e-2_f64) * t43816 + F::cast_from(0.24722222222222222223e-1_f64) * t63361 + F::cast_from(0.12361111111111111111e-1_f64) * t63382 + F::cast_from(0.37083333333333333333e-1_f64) * t63384 - F::cast_from(0.37083333333333333334e-1_f64) * t63398 - F::cast_from(0.55625000000000000001e-1_f64) * t63400;
    t71308
}
