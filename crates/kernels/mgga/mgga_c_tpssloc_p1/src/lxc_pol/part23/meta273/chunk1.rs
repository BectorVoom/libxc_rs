//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 955/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk955<F: Float>(t1363: F, t16211: F, t1831: F, t19834: F, t19839: F, t19841: F, t19851: F, t19904: F, t20433: F, t20442: F, t20484: F, t20508: F, t20599: F, t3803: F, t5240: F, t6427: F, t6431: F) -> F {
    let t20601 = -F::new(119.0) / F::new(4608.0) * t16211 - t5240 * t6431 / F::new(256.0) + F::new(5.0) / F::new(256.0) * t5240 * t6427 - F::new(5.0) / F::new(128.0) * t1363 * t20433 - t19904 * t1831 / F::new(256.0) - F::new(7.0) / F::new(1536.0) * t19834 - F::new(7.0) / F::new(16.0) * t19839 + F::new(7.0) / F::new(48.0) * t19841 - t3803 * t20442 / F::new(1024.0) - F::new(7.0) / F::new(768.0) * t19851 + t20484 + t20508 + t20599;
    t20601
}
