//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2717/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2717<F: Float>(t39845: F, t54456: F, t39615: F, t39642: F, t39655: F, t39658: F, t39844: F, t57203: F, t57204: F, t57205: F, t57206: F, t57207: F, t57209: F, t57210: F, t57212: F, t57213: F, t57214: F) -> (F, F, F) {
    let t57215 = F::cast_from(120.0_f64) * t39845;
    let t57216 = F::cast_from(48.0_f64) * t54456;
    let t57217 = -t57203 - t57204 - t57205 + t39615 + t57206 + t57207 + t57209 + t57210 + t57212 + t39642 - t57213 + t57214 - t39655 - t39658 + t39844 + t57215 - t57216;
    (t57215, t57216, t57217)
}
