//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta330<F: Float>(t2371: F, t3691: F, t1294: F, t9494: F, t2535: F, t215: F, t535: F, t9569: F, t1314: F, t2559: F, t1317: F, t795: F, t9580: F) -> (F, F, F, F, F, F, F) {
        let (t12138, t12141, t12142, t12188, t12189, t12190, t12194) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1261::<F>(t2371, t3691, t1294, t9494, t2535, t215, t535, t9569, t1314, t2559, t1317, t795, t9580);
    (t12138, t12141, t12142, t12188, t12189, t12190, t12194)
}
