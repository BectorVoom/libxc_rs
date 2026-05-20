//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2451/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2451<F: Float>(t11065: F, t42387: F, t1005: F, t10375: F, t10475: F, t42342: F, t42345: F, t2770: F, t283: F, t11064: F, t42332: F, t11058: F) -> (F, F, F, F, F, F) {
    let t43361 = t11065 * t42387;
    let t43382 = t1005 * t10375;
    let t43385 = t42342 * t10475 * t42345;
    let t43398 = F::new(1.0) / t283 / t2770;
    let t43470 = t42332 * t11064;
    let t43473 = t42332 * t11058;
    (t43361, t43382, t43385, t43398, t43470, t43473)
}
