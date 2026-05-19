//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 990/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk990<F: Float>(t114932: F, t112915: F, t112920: F, t112927: F, t112932: F, t112936: F, t112942: F, t114913: F, t114916: F, t114926: F, t2053: F, t22978: F, t23190: F, t23278: F, t23281: F, t24305: F, t25168: F, t26728: F, t2713: F, t2718: F, t31400: F, t6632: F, t7092: F, t7107: F, t855: F) -> F {
    let t114933 = F::cast_from(0.82246703342411321824e-2_f64) * t114932;
    let t114934 = F::new(2.0) * t855 * t2718 * t2053 * t23190 + F::new(4.0) * t23281 * t7092 - t112915 + F::new(4.0) * t24305 * t6632 - t112920 + F::cast_from(0.49348022005446793095e-1_f64) * t114913 + t112927 - t112932 + F::cast_from(0.16449340668482264365e-1_f64) * t114916 - F::new(2.0) * t23281 * t7107 - F::new(12.0) * t25168 * t26728 * t22978 - F::new(2.0) * t2713 * t31400 - F::cast_from(0.16449340668482264365e-1_f64) * t114926 + t112936 + F::new(4.0) * t23278 * t7092 - t114933 - t112942;
    t114934
}
