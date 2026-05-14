//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1277/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1277<F: Float>(t104911: F, t104953: F, t104958: F, t106826: F, t106829: F, t106853: F, t106862: F, t2110: F, t24514: F, t27341: F, t27961: F, t27966: F, t27972: F, t27976: F, t7432: F, t7975: F, t7978: F, t96045: F) -> (F,) {
    let t109004 = t104911 * t106853 - 15.0 * t96045 * t27961 - 15.0 * t24514 * t106826 + 5.0 / 2.0 * t104953 * t7432 + 5.0 * t104958 * t7432 + 5.0 * t27341 * t27972 + 5.0 / 2.0 * t27341 * t27976 + t106862 * t2110 + t106829 * t2110 + 2.0 * t27966 * t7975 + 2.0 * t27966 * t7978;
    (t109004,)
}
