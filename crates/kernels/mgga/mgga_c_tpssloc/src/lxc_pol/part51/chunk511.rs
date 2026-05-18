//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 511/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk511<F: Float>(t964: F, t995: F, t1050: F, t225: F, t1053: F, t386: F, t68: F) -> (F, F, F, F) {
    let t3163 = t964 * t995;
    let t3169 = t1050 * t225;
    let t3173 = F::new(1.0) / t1053 / t386;
    let t3174 = t68 * t3173;
    (t3163, t3169, t3173, t3174)
}
