//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1867;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta618<F: Float>(t12020: F, t1385: F, t1992: F, t22635: F, t6439: F, t28117: F, t81159: F, t1377: F, t6330: F, t26331: F, t26332: F, t5187: F, t19885: F, t90915: F, t91004: F, t28135: F, t6914: F, t550: F, t57607: F, t6976: F, t28168: F, t57704: F, t562: F, t6347: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t96910, t96920, t96925, t96929) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1867::<F>(t12020, t1385, t1992, t22635, t6439, t28117, t81159, t1377, t6330, t26331, t26332, t5187);
        let (t96935, t96937, t96941, t96945, t96949, t96951) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1868::<F>(t19885, t90915, t91004, t28135, t6914, t1992, t550, t57607, t6976, t28168, t57704, t562, t6347);
    (t96910, t96920, t96925, t96929, t96935, t96937, t96941, t96945, t96949, t96951)
}
