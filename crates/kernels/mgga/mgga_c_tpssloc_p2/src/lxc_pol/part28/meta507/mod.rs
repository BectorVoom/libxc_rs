//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta507<F: Float>(t3701: F, t7216: F, t31: F, t63: F, t607: F, t7939: F, t1390: F, t22811: F, t2233: F, t2239: F, t601: F, t9238: F) -> (F, F, F, F, F, F, F) {
        let (t32193, t32332, t33899, t34711, t39041, t39049, t39054) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1755::<F>(t3701, t7216, t31, t63, t607, t7939, t1390, t22811, t2233, t2239, t601, t9238);
    (t32193, t32332, t33899, t34711, t39041, t39049, t39054)
}
