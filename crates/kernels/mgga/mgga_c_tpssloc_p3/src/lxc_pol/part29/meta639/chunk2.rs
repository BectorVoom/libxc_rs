//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2102/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2102<F: Float>(t13377: F, t1880: F, t1894: F, t214: F, t22984: F, t22992: F, t22993: F, t23009: F, t25297: F, t2617: F, t4166: F, t4234: F, t812: F, t81571: F, t81592: F, t87055: F, t87059: F, t87067: F, t87068: F, t87073: F, t87076: F, t87078: F, t87080: F, t87084: F) -> F {
    let t87092 = t1880 * t214 * t1894 * t13377;
    let t87094 = F::cast_from(0.3289868133696452873e-1_f64) * t87055 - F::cast_from(0.9869604401089358619e-1_f64) * t87059 - F::new(2.0) * t2617 * t25297 - F::new(2.0) * t812 * t22992 * t4234 + t87067 - F::cast_from(0.26044789391763585244e-1_f64) * t87068 - F::cast_from(0.41123351671205660912e-2_f64) * t81571 - F::new(2.0) * t4166 * t22993 + t87073 - F::cast_from(0.49348022005446793095e-1_f64) * t87076 - F::cast_from(0.2302907693587517011e0_f64) * t87078 + F::cast_from(0.63969658155208805863e-1_f64) * t87080 + F::cast_from(0.3289868133696452873e-1_f64) * t87084 - t4166 * t22984 + F::new(2.0) * t4166 * t23009 - F::cast_from(0.76763589786250567036e-1_f64) * t81592 + F::cast_from(0.82246703342411321825e-2_f64) * t87092;
    t87094
}
