//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1054/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1054<F: Float>(t39451: F, t2604: F, t35106: F, t35110: F, t35114: F, t35118: F, t39440: F, t39445: F, t39449: F, t39453: F, t39455: F, t39457: F, t39461: F, t39463: F, t39465: F, t39470: F, t39474: F, t9620: F) -> F {
    let t42970 = F::cast_from(0.3193131120497015617e0_f64) * t39451;
    let t42985 = -F::cast_from(0.47885174879960069324e-4_f64) * t39440 - F::cast_from(0.638468998399467591e-4_f64) * t39445 - F::cast_from(0.212822999466489197e-4_f64) * t39449 + t42970 + F::cast_from(0.5107751987195740728e-4_f64) * t39453 - F::cast_from(0.1064114997332445985e-4_f64) * t39455 - F::cast_from(0.212822999466489197e-4_f64) * t39457 + F::cast_from(0.23948483403727617128e0_f64) * t2604 * t9620 - F::cast_from(0.5107751987195740728e-4_f64) * t39461 + F::cast_from(0.5107751987195740728e-4_f64) * t39463 + F::cast_from(0.638468998399467591e-4_f64) * t39465 - F::cast_from(0.212822999466489197e-4_f64) * t39470 + F::cast_from(0.85129199786595678799e-5_f64) * t39474 - F::cast_from(0.30487649791575028312e-3_f64) * t35106 + F::cast_from(0.43368970657079495308e-4_f64) * t35110 - F::cast_from(0.60975299583150056624e-3_f64) * t35114 + F::cast_from(0.86737941314158990616e-4_f64) * t35118;
    t42985
}
