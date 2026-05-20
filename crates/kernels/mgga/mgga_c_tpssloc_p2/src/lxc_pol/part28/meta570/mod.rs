//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta570<F: Float>(t7524: F, t81612: F, t81613: F, t4240: F, t81865: F, t4191: F, t13302: F, t23146: F, t13322: F, t4250: F, t13316: F, t13312: F) -> (F, F, F, F, F, F, F, F) {
        let (t87177, t87183, t87185, t87187, t87189, t87191, t87193, t87195) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1850::<F>(t7524, t81612, t81613, t4240, t81865, t4191, t13302, t23146, t13322, t4250, t13316, t13312);
    (t87177, t87183, t87185, t87187, t87189, t87191, t87193, t87195)
}
