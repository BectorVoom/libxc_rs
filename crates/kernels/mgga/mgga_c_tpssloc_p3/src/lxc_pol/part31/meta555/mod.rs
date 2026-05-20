//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta555<F: Float>(t81153: F, t81317: F, t81398: F, t531: F, t7216: F, t2056: F, t40772: F, t193: F, t201: F, t7109: F, t10143: F, t82069: F) -> (F, F, F, F, F, F, F, F) {
        let (t84597, t84659, t84705, t84733, t84766, t84797, t84800, t84820) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1783::<F>(t81153, t81317, t81398, t531, t7216, t2056, t40772, t193, t201, t7109, t10143, t82069);
    (t84597, t84659, t84705, t84733, t84766, t84797, t84800, t84820)
}
