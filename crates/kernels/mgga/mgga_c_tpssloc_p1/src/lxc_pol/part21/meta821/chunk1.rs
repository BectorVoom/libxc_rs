//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2887/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2887<F: Float>(t59698: F, t60243: F, t60245: F, t60248: F, t60251: F, t60254: F, t60257: F, t60260: F, t60263: F, t60265: F, t60267: F, t60269: F, t60271: F, t60274: F, t60277: F) -> F {
    let t60279 = F::new(0.16504875e0) * t60243 + F::new(0.82524375e-1) * t60245 - F::cast_from(0.485484375e1_f64) * t60248 + F::new(0.19419375e1) * t60251 + F::cast_from(0.6189328125e-1_f64) * t60254 - F::cast_from(0.412621875e-1_f64) * t60257 - F::new(0.258925e1) * t60260 + F::new(0.16504875e0) * t60263 + F::new(0.19419375e1) * t60265 - F::new(0.258925e1) * t60267 - F::new(0.1294625e1) * t60269 - F::cast_from(0.412621875e-1_f64) * t60271 + F::new(0.60385e0) * t59698 + F::cast_from(0.36793333333333333334e-1_f64) * t60274 + F::new(0.33114e0) * t60277;
    t60279
}
