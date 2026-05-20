//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1945;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta660<F: Float>(t16918: F, t23146: F, t16898: F, t4191: F, t87199: F, t4240: F, t232: F, t58569: F, t6605: F, t815: F, t2628: F, t5585: F, t828: F, t16949: F, t221: F, t25154: F, t25119: F, t841: F, t25038: F, t25248: F, t776: F, t98422: F, t23110: F, t23185: F, t28321: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98847, t98849, t98851, t98853, t98858, t98862) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1945::<F>(t16918, t23146, t16898, t4191, t87199, t4240, t232, t58569, t6605, t815, t2628, t5585, t828);
        let (t98868, t98871, t98881, t98884) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1946::<F>(t16949, t221, t25154, t25119, t841, t25038, t25248, t776, t98422, t23110, t23185, t28321);
    (t98847, t98849, t98851, t98853, t98858, t98862, t98868, t98871, t98881, t98884)
}
