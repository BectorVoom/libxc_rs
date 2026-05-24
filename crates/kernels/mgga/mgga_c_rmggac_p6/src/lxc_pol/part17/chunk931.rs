//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 931/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk931<F: Float>(t2320: F, t38374: F, t1953: F, t2127: F, t39406: F, t39452: F, t45466: F, t45469: F, t45473: F, t45477: F, t45482: F, t45484: F, t45486: F, t45488: F, t45493: F, t45495: F, t45499: F, t45503: F, t45505: F, t45507: F, t72: F) -> F {
    let t45509 = t38374 * t2320;
    let t45511 = t39406 + F::cast_from(0.20455996240684006296e-1_f64) * t45466 + F::cast_from(0.20455996240684006296e-1_f64) * t45469 + t72 * t1953 * t2127 - F::cast_from(0.24829349937757072983e-4_f64) * t45473 - F::cast_from(0.42564599893297839398e-5_f64) * t45477 + F::cast_from(0.53205749866622299248e-5_f64) * t45482 + F::cast_from(0.85129199786595678796e-5_f64) * t45484 - F::cast_from(0.24829349937757072983e-4_f64) * t45486 - F::cast_from(0.19863479950205658386e-4_f64) * t45488 + F::cast_from(0.76616279807936110914e-4_f64) * t45493 + F::cast_from(0.25538759935978703638e-4_f64) * t45495 - F::cast_from(0.1064114997332445985e-4_f64) * t45499 - F::cast_from(0.25538759935978703638e-4_f64) * t45503 + t39452 + F::cast_from(0.99317399751028291929e-5_f64) * t45505 - F::cast_from(0.1064114997332445985e-4_f64) * t45507 - F::cast_from(0.1064114997332445985e-4_f64) * t45509;
    t45511
}
