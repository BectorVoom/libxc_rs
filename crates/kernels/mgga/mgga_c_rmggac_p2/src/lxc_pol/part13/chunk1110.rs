//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1110/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1110<F: Float>(t41114: F, t41128: F, t118: F, t338: F, t35918: F, t35922: F, t35926: F, t35937: F, t41086: F, t41095: F, t41101: F, t41106: F, t41108: F, t41120: F, t43637: F) -> F {
    let t44143 = F::cast_from(0.15965655602485078085e0_f64) * t41114;
    let t44145 = F::cast_from(0.3193131120497015617e0_f64) * t41128;
    let t44146 = -F::cast_from(0.11974241701863808564e0_f64) * t41086 + F::cast_from(0.19957069503106347607e-1_f64) * t118 * t338 * t43637 - F::cast_from(0.11974241701863808564e0_f64) * t41095 - F::cast_from(0.35922725105591425692e0_f64) * t41101 + F::cast_from(0.71845450211182851384e0_f64) * t41106 + F::cast_from(0.17961362552795712846e0_f64) * t41108 + F::cast_from(0.95793933614910468511e0_f64) * t35918 + F::cast_from(0.1333427903096438929e0_f64) * t35922 + F::cast_from(0.53337116123857557162e0_f64) * t35926 + F::cast_from(0.36366215538993788974e-1_f64) * t35937 + t44143 + F::cast_from(0.95793933614910468511e0_f64) * t41120 - t44145;
    t44146
}
