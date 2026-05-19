//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 981/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk981<F: Float>(t11399: F, t1147: F, t1156: F, t1164: F, t3411: F, t3419: F, t3423: F, t11203: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F, t11221: F, t11224: F, t11230: F, t11233: F) -> (F, F, F, F) {
    let t11478 = t1147 * t11399 * t1156;
    let t11480 = F::cast_from(0.5848223622634646207e0_f64) * t1164 * t11478;
    let t11482 = F::cast_from(0.17544670867903938621e1_f64) * t3411 * t3419;
    let t11484 = F::cast_from(0.51947577317044391276e2_f64) * t3411 * t3423;
    let t11487 = F::new(20.0) / F::new(27.0) * t11203;
    let t11496 = t11487 - F::new(5.0) / F::new(9.0) * t11211 - t11213 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t11215 + t11217 / F::new(3.0) - F::new(2.0) / F::new(27.0) * t11221 + t11224 / F::new(3.0) + t11230 / F::new(6.0) - t11206 - t11233 - t11209 / F::new(6.0);
    (t11480, t11482, t11484, t11496)
}
