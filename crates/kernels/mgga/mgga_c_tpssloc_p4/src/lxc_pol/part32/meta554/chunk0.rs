//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1914/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1914<F: Float>(t1874: F, t28002: F, t4028: F, t7461: F, t19451: F, t1774: F, t7467: F, t652: F, t2006: F, t6361: F, t1807: F, t7722: F) -> (F, F, F, F, F, F, F) {
    let t28038 = F::new(4.0) * t28002 * t1874;
    let t28040 = F::new(4.0) * t4028 * t7461;
    let t28042 = F::new(2.0) * t19451 * t1874;
    let t28045 = t1774 * t7467;
    let t28047 = F::new(4.0) * t652 * t28045;
    let t28051 = t6361 * t2006;
    let t28053 = t1807 * t7722;
    (t28038, t28040, t28042, t28045, t28047, t28051, t28053)
}
