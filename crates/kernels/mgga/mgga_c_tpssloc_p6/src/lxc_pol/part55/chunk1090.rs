//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1090/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1090<F: Float>(t1814: F, t8465: F, t8467: F, t5248: F, t5249: F, t550: F, t31170: F, t1831: F, t8466: F, t31154: F, t31161: F, t31178: F, t32712: F, t32715: F) -> (F, F, F) {
    let t32717 = t1814 * t8465;
    let t32718 = t32717 * t8467;
    let t32721 = t5248 * t5249 * t550;
    let t32722 = t31170 * t32721;
    let t32724 = t8466 * t1831;
    let t32726 = -t31154 - F::cast_from(0.48447307312968469025e-2_f64) * t32712 - t31161 - F::cast_from(0.80745512188280781708e-3_f64) * t32715 + t32718 / F::new(1536.0) - t32722 / F::new(1536.0) - t31178 - t32724 / F::new(384.0);
    (t32717, t32721, t32726)
}
