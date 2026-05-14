//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1093/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1093<F: Float>(t23384: F, t6707: F, t6695: F, t6680: F, t6683: F, t6699: F, t968: F, t1920: F, t225: F, t3173: F, t3175: F, t6704: F, t1922: F, t3010: F, t2776: F, t6690: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23385 = t23384 * t6707;
    let t23387 = t23384 * t6695;
    let t23389 = t6680 * t6683;
    let t23391 = t968 * t6699;
    let t23392 = t1920 * t23391;
    let t23394 = t225 * t3173;
    let t23395 = t23394 * t3175;
    let t23396 = t6704 * t23395;
    let t23399 = t3010 * t1922;
    let t23402 = t6690 * t2776;
    (t23385, t23387, t23389, t23391, t23392, t23394, t23395, t23396, t23399, t23402)
}
