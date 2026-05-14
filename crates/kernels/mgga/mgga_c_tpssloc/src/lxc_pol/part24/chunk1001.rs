//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1001/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1001<F: Float>(t1995: F, t67: F, t246: F, t3734: F, t550: F, t3806: F, t12368: F, t3805: F, t3807: F, t3777: F, t3802: F, t12392: F, t12395: F, t12397: F, t12404: F, t12409: F, t12413: F, t1341: F, t1354: F, t3778: F, t3783: F, t3803: F, t3809: F, t3853: F, t3872: F) -> (F, F, F) {
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12420 = t550 * t3734;
    let t12422 = t12419 * t3806 * t12420;
    let t12426 = t3805 * t12368 * t3807;
    let t12429 = t3777 * t3802;
    let t12432 = -t1341 * t12392 / 3072.0 + 7.0 / 1536.0 * t12395 - t12397 * t1354 / 1024.0 - t3778 * t3853 / 1024.0 + t3803 * t12404 / 256.0 + t3803 * t12409 / 256.0 - t3803 * t12413 / 1024.0 + 5.0 / 256.0 * t3783 * t3872 - 5.0 / 256.0 * t3803 * t12422 + t3803 * t12426 / 256.0 + t12429 * t3809 / 128.0;
    (t12422, t12426, t12432)
}
