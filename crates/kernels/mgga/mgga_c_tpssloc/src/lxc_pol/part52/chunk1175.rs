//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1175/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1175<F: Float>(t31181: F, t539: F, t225: F, t8471: F, t6883: F, t8480: F, t2006: F, t552: F, t1307: F, t6637: F, t6888: F, t794: F, t8479: F) -> (F, F, F, F, F, F, F, F) {
    let t31182 = t539 * t31181;
    let t31189 = t8471 * t225;
    let t31192 = F::cast_from(0.38381794893125283518e-1_f64) * t6883 * t8480;
    let t31193 = t552 * t2006;
    let t31194 = t31193 * t1307;
    let t31195 = t6637 * t31194;
    let t31197 = F::cast_from(0.3289868133696452873e-1_f64) * t6888 * t31195;
    let t31198 = t794 * t8479;
    (t31182, t31189, t31192, t31193, t31194, t31195, t31197, t31198)
}
