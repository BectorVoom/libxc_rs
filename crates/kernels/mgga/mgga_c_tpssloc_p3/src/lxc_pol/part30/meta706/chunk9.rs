//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2329/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2329<F: Float>(t100637: F, t100818: F, t113: F, t20100: F, t20136: F, t510: F, t6517: F, t96654: F, t97910: F, t97914: F, t97916: F, t97919: F, t97923: F, t97925: F, t97928: F, t97930: F, t97932: F, t97935: F, t97937: F, t97941: F, t97942: F, t97947: F, t97949: F) -> F {
    let t100822 = -t97910 + t97914 - t97916 - t97919 + t97923 + t97925 - t97928 + t97930 - t97932 - t97935 - t97937 - F::new(2.0) * t6517 * t20100 + t97941 + t97942 - F::new(4.0) * t6517 * t20136 - t97947 - t97949 - t113 * (t100637 + t100818) - t96654 * t510;
    t100822
}
