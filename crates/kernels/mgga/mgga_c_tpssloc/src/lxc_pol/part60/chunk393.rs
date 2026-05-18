//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 393/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk393<F: Float>(t3031: F, t3032: F, t371: F, t335: F, t368: F, t1015: F, t1043: F, t121: F, t283: F, t883: F, t61: F, t363: F) -> (F, F, F, F, F, F, F, F) {
    let t3033 = t3031 * t3032;
    let t3034 = t371 * t371;
    let t3036 = F::new(1.0) / t3034 / t335;
    let t3037 = t368 * t3036;
    let t3038 = t1015 * t3037;
    let t3039 = t3033 * t3038;
    let t3051 = t121 * t1043;
    let t3061 = F::new(1.0) / t283 / t883;
    let t3062 = t61 * t3061;
    let t3067 = t363 * t368;
    (t3033, t3034, t3036, t3037, t3039, t3051, t3062, t3067)
}
