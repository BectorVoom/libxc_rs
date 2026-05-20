//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2748/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2748<F: Float>(t57947: F, t12971: F, t2522: F, t39397: F, t39400: F, t39408: F, t39411: F, t40708: F, t4310: F, t4314: F, t4315: F, t57932: F, t57936: F, t57939: F, t57943: F, t57946: F, t776: F) -> (F, F) {
    let t57948 = F::new(8.0) * t57947;
    let t57955 = F::new(6.0) * t12971 * t2522 * t4310 + F::new(12.0) * t12971 * t4314 * t4315 + F::new(6.0) * t2522 * t57932 * t776 - t39397 - t39400 + t39408 + t39411 + t40708 + t57936 + t57939 + t57943 + t57946 + t57948;
    (t57948, t57955)
}
