//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 942/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk942<F: Float>(t1933: F, t25588: F, t4603: F, t6717: F, t1597: F, t1934: F, t1025: F, t1046: F, t1607: F, t1618: F, t1920: F, t1937: F, t23419: F, t23422: F, t23425: F, t23437: F, t25571: F, t25574: F, t25577: F, t25580: F, t25585: F, t4575: F, t4579: F, t6735: F) -> (F,) {
    let t25589 = t1933 * t25588;
    let t25598 = t6717 * t4603;
    let t25600 = t1934 * t1597;
    let t25601 = t1933 * t25600;
    let t25605 = -t1920 * t25571 / 144.0 + t1920 * t25574 / 216.0 + t25577 * t1025 / 1536.0 + t25580 * t1046 / 2304.0 - t23437 * t1618 / 288.0 - 0.80745512188280781712e-3 * t25585 * t1937 + 0.10093189023535097714e-3 * t25589 * t1937 + t23419 * t4575 / 2304.0 + t23419 * t4579 / 2304.0 - t23422 * t1607 / 108.0 + t25598 / 864.0 - 0.10093189023535097714e-3 * t25601 * t6735 + t23425 / 864.0;
    (t25605,)
}
