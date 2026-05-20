//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1050/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1050<F: Float>(t3777: F, t3802: F, t12392: F, t12395: F, t12397: F, t12404: F, t12409: F, t12413: F, t12422: F, t12426: F, t1341: F, t1354: F, t3778: F, t3783: F, t3803: F, t3809: F, t3853: F, t3872: F) -> F {
    let t12429 = t3777 * t3802;
    let t12432 = -t1341 * t12392 / F::new(3072.0) + F::new(7.0) / F::new(1536.0) * t12395 - t12397 * t1354 / F::new(1024.0) - t3778 * t3853 / F::new(1024.0) + t3803 * t12404 / F::new(256.0) + t3803 * t12409 / F::new(256.0) - t3803 * t12413 / F::new(1024.0) + F::new(5.0) / F::new(256.0) * t3783 * t3872 - F::new(5.0) / F::new(256.0) * t3803 * t12422 + t3803 * t12426 / F::new(256.0) + t12429 * t3809 / F::new(128.0);
    t12432
}
