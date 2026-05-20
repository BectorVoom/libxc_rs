//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2070/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2070<F: Float>(t90912: F, t215: F, t22839: F, t562: F, t80854: F, t1338: F, t26328: F, t26462: F, t6914: F, t22705: F, t26414: F, t81228: F) -> (F, F, F, F, F, F) {
    let t90913 = F::cast_from(0.76763589786250567036e-1_f64) * t90912;
    let t90914 = t22839 * t215;
    let t90915 = t80854 * t562;
    let t90952 = t1338 * t26328;
    let t90956 = t6914 * t26462;
    let t90957 = F::cast_from(0.38381794893125283518e-1_f64) * t90956;
    let t90961 = t81228 * t22705 * t26414;
    (t90913, t90914, t90915, t90952, t90957, t90961)
}
