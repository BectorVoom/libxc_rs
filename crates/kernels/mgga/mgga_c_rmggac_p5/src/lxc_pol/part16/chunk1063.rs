//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1063/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1063<F: Float>(t44615: F, t44620: F, t44627: F, t44632: F, t44637: F, t44642: F, t44647: F, t44651: F, t44656: F, t44662: F, t44668: F, t44670: F, t44676: F, t44682: F, t44684: F, t44690: F, t44692: F) -> F {
    let t48157 = -F::cast_from(0.1702583995731913576e-4_f64) * t44615 + F::cast_from(0.15323255961587222184e-3_f64) * t44620 + F::cast_from(0.212822999466489197e-4_f64) * t44627 + F::cast_from(0.85129199786595678799e-5_f64) * t44632 - F::cast_from(0.2553875993597870364e-4_f64) * t44637 + F::cast_from(0.2553875993597870364e-4_f64) * t44642 + F::cast_from(0.85129199786595678799e-5_f64) * t44647 - F::cast_from(0.1702583995731913576e-4_f64) * t44651 - F::cast_from(0.85129199786595678799e-5_f64) * t44656 - F::cast_from(0.5107751987195740728e-4_f64) * t44662 + F::cast_from(0.212822999466489197e-4_f64) * t44668 + F::cast_from(0.1702583995731913576e-4_f64) * t44670 + F::cast_from(0.1702583995731913576e-4_f64) * t44676 + F::cast_from(0.1702583995731913576e-4_f64) * t44682 - F::cast_from(0.5107751987195740728e-4_f64) * t44684 - F::cast_from(0.5107751987195740728e-4_f64) * t44690 - F::cast_from(0.1702583995731913576e-4_f64) * t44692;
    t48157
}
