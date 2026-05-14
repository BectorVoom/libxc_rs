//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 925/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk925<F: Float>(t671: F, t8710: F, t32255: F, t33103: F, t116905: F, t116910: F, t116917: F, t116920: F, t116929: F, t116936: F, t116945: F, t116954: F, t119880: F, t119902: F, t119917: F, t119924: F, t119928: F, t119932: F, t119933: F, t119948: F, t32245: F, t32249: F, t33111: F, t8706: F) -> (F, F) {
    let t124293 = t8710 * t671;
    let t124324 = t33103 * t32255;
    let t124330 = -40.0 / 27.0 * t116920 + t116917 - 20.0 / 27.0 * t116945 + 40.0 / 9.0 * t116936 + 80.0 / 27.0 * t116910 - 5.0 / 3.0 * t32245 * t119917 - 5.0 / 9.0 * t116929 * t33111 - 5.0 / 9.0 * t32249 * t119924 - 5.0 / 9.0 * t32249 * t119928 + 10.0 / 9.0 * t119932 * t8706 * t119933 - 5.0 / 3.0 * t32245 * t119948 - 20.0 / 27.0 * t124324 - 10.0 / 9.0 * t116954 * t119902 + 10.0 / 3.0 * t116905 * t119880;
    (t124293, t124330)
}
