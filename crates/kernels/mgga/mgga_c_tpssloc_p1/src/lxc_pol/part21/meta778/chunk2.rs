//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2692/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2692<F: Float>(t19767: F, t40409: F, t19771: F, t3726: F, t12199: F, t19775: F, t40387: F, t40401: F, t40402: F, t40404: F, t40407: F, t40410: F, t40422: F, t40425: F, t54663: F, t54667: F, t54671: F) -> F {
    let t56535 = t40409 * t19767;
    let t56537 = t3726 * t19771;
    let t56539 = t12199 * t19775;
    let t56542 = F::cast_from(0.38888888888888888889e-1_f64) * t40387 - t40401 + F::cast_from(0.11234567901234567901e0_f64) * t40402 - F::cast_from(0.12962962962962962963e-1_f64) * t40404 + F::cast_from(0.6574074074074074074e-1_f64) * t40407 + F::cast_from(0.15833333333333333333e-1_f64) * t40410 + t40422 - F::cast_from(0.52777777777777777776e-2_f64) * t40425 - F::cast_from(0.39999999999999999998e-1_f64) * t54663 + F::cast_from(0.66666666666666666664e-2_f64) * t54667 + F::cast_from(0.15833333333333333333e-1_f64) * t56535 + F::cast_from(0.77777777777777777774e-2_f64) * t56537 - F::cast_from(0.52777777777777777776e-2_f64) * t56539 + F::cast_from(0.93333333333333333328e-1_f64) * t54671;
    t56542
}
