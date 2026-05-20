//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2144/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2144<F: Float>(t26395: F, t5187: F, t6637: F, t6888: F, t22892: F, t22893: F, t28148: F, t1336: F, t19732: F, t19815: F, t28178: F, t3777: F, t6987: F, t6988: F, t81080: F, t90957: F, t90962: F, t90964: F, t97036: F, t97040: F, t97043: F, t97046: F, t97049: F, t97055: F, t97059: F, t97063: F) -> F {
    let t97067 = t6888 * t6637 * t26395 * t5187;
    let t97070 = t22892 * t22893 * t28148;
    let t97075 = -F::cast_from(0.52089578783527170488e-1_f64) * t81080 - F::cast_from(0.16449340668482264365e-1_f64) * t97036 - F::cast_from(0.16449340668482264365e-1_f64) * t97040 - F::cast_from(0.16449340668482264365e-1_f64) * t97043 + F::cast_from(0.49348022005446793095e-1_f64) * t97046 - F::cast_from(0.82246703342411321825e-2_f64) * t97049 - t1336 * t6987 * t19732 + F::cast_from(0.82246703342411321825e-2_f64) * t97055 - F::cast_from(0.49348022005446793095e-1_f64) * t97059 - F::cast_from(0.3289868133696452873e-1_f64) * t97063 - F::cast_from(0.3289868133696452873e-1_f64) * t97067 + F::cast_from(0.16449340668482264365e-1_f64) * t97070 - F::new(2.0) * t3777 * t28178 - t19815 * t6988 + t90957 - t90962 - t90964;
    t97075
}
