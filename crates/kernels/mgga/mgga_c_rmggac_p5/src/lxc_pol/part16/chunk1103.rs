//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1103/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1103<F: Float>(t10270: F, t2604: F, t37904: F, t40623: F, t43654: F, t47156: F, t47158: F, t47162: F, t47167: F, t47173: F, t47175: F, t47178: F, t47180: F, t47182: F, t47188: F, t6434: F, t6449: F, t6522: F, t699: F, t739: F, t8264: F, t903: F) -> F {
    let t48924 = F::cast_from(0.20431007948782962912e-3_f64) * t47156 + F::cast_from(0.10215503974391481456e-3_f64) * t47158 + F::cast_from(0.5107751987195740728e-4_f64) * t47162 - F::cast_from(0.5107751987195740728e-4_f64) * t47167 + F::cast_from(0.3405167991463827152e-4_f64) * t47173 - F::cast_from(0.58540737209111952978e0_f64) * t40623 - F::cast_from(0.23948483403727617128e0_f64) * t47175 + F::cast_from(0.23948483403727617128e0_f64) * t739 * t8264 * t6522 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t699 * t6449 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t699 * t6434 + F::cast_from(0.23948483403727617128e0_f64) * t2604 * t10270 + F::cast_from(0.39726959900411316773e-4_f64) * t47178 + F::cast_from(0.39726959900411316773e-4_f64) * t47180 + t43654 + F::cast_from(0.11918087970123395032e-3_f64) * t47182 + F::cast_from(0.15965655602485078085e0_f64) * t47188 + t37904;
    t48924
}
