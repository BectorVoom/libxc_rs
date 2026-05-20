//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2910/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2910<F: Float>(t59698: F, t60243: F, t60245: F, t60248: F, t60251: F, t60254: F, t60257: F, t60260: F, t60263: F, t60265: F, t60267: F, t60269: F, t60271: F, t60274: F, t60277: F) -> F {
    let t60665 = F::new(0.6311625e0) * t60243 + F::new(0.31558125e0) * t60245 - F::cast_from(0.6618234375e1_f64) * t60248 + F::cast_from(0.264729375e1_f64) * t60251 + F::cast_from(0.2366859375e0_f64) * t60254 - F::cast_from(0.157790625e0_f64) * t60257 - F::new(0.3529725e1) * t60260 + F::new(0.6311625e0) * t60263 + F::cast_from(0.264729375e1_f64) * t60265 - F::new(0.3529725e1) * t60267 - F::new(0.17648625e1) * t60269 - F::cast_from(0.157790625e0_f64) * t60271 + F::new(0.103295e1) * t59698 + F::cast_from(0.46308888888888888889e-1_f64) * t60274 + F::new(0.41678e0) * t60277;
    t60665
}
