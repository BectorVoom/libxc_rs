//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2901/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2901<F: Float>(t59698: F, t60243: F, t60245: F, t60248: F, t60251: F, t60254: F, t60257: F, t60260: F, t60263: F, t60265: F, t60267: F, t60269: F, t60271: F, t60274: F, t60277: F) -> F {
    let t60529 = F::cast_from(0.3071625e0_f64) * t60243 + F::cast_from(0.15358125e0_f64) * t60245 - F::cast_from(0.3560484375e1_f64) * t60248 + F::cast_from(0.142419375e1_f64) * t60251 + F::cast_from(0.1151859375e0_f64) * t60254 - F::cast_from(0.76790625e-1_f64) * t60257 - F::cast_from(0.1898925e1_f64) * t60260 + F::cast_from(0.3071625e0_f64) * t60263 + F::cast_from(0.142419375e1_f64) * t60265 - F::cast_from(0.1898925e1_f64) * t60267 - F::cast_from(0.9494625e0_f64) * t60269 - F::cast_from(0.76790625e-1_f64) * t60271 + F::cast_from(0.59793333333333333334e0_f64) * t59698 + F::cast_from(0.36514074074074074074e-1_f64) * t60274 + F::cast_from(0.32862666666666666666e0_f64) * t60277;
    t60529
}
