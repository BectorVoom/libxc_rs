//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1946;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta623<F: Float>(t16235: F, t91361: F, t5303: F, t80820: F, t16356: F, t6916: F, t16018: F, t1998: F, t236: F, t6926: F, t1339: F, t54153: F, t550: F, t6936: F, t16311: F, t3788: F, t3850: F, t57554: F, t26233: F, t3858: F, t22783: F, t5310: F, t22760: F, t5234: F, t3795: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91362, t91364, t91366, t91370, t91374) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1946::<F>(t16235, t91361, t5303, t80820, t16356, t6916, t16018, t1998, t236, t6926, t1339, t54153, t550, t6936);
        let (t91378, t91381, t91384, t91386, t91389) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1947::<F>(t16311, t3788, t3850, t6936, t57554, t26233, t3858, t22783, t5310, t22760, t5234, t3795);
    (t91362, t91364, t91366, t91370, t91374, t91378, t91381, t91384, t91386, t91389)
}
