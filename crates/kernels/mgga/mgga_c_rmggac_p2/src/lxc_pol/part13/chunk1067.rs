//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1067/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1067<F: Float>(t40062: F, t40075: F, t40084: F, t40086: F, t40088: F, t37768: F, t40068: F, t40073: F, t40082: F, t40093: F, t40098: F, t40102: F, t40106: F, t40110: F, t40112: F, t40114: F, t40116: F) -> F {
    let t43267 = F::cast_from(0.39726959900411316772e-4_f64) * t40062;
    let t43270 = F::cast_from(0.49658699875514145965e-4_f64) * t40075;
    let t43272 = F::cast_from(0.3842256877732895568e-2_f64) * t40084;
    let t43273 = F::cast_from(0.3842256877732895568e-2_f64) * t40086;
    let t43274 = F::cast_from(0.3842256877732895568e-2_f64) * t40088;
    let t43283 = -t43267 + F::cast_from(0.1064114997332445985e-4_f64) * t40068 - F::cast_from(0.23942587439980034662e-4_f64) * t40073 - t43270 - t37768 + F::cast_from(0.5107751987195740728e-4_f64) * t40082 + t43272 + t43273 + t43274 + F::cast_from(0.5107751987195740728e-4_f64) * t40093 - F::cast_from(0.212822999466489197e-4_f64) * t40098 - F::cast_from(0.5107751987195740728e-4_f64) * t40102 + F::cast_from(0.61293023846348888736e-3_f64) * t40106 + F::cast_from(0.15323255961587222184e-3_f64) * t40110 - F::cast_from(0.2553875993597870364e-4_f64) * t40112 - F::cast_from(0.1702583995731913576e-4_f64) * t40114 - F::cast_from(0.5107751987195740728e-4_f64) * t40116;
    t43283
}
