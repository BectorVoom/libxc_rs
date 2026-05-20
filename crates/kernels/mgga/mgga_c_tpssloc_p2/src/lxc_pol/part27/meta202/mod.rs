//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta202 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta202<F: Float>(t344: F, t4509: F, t4338: F, t1409: F, t2989: F, t2988: F, t2987: F, t4343: F, t3966: F, t978: F, t977: F, t135: F, t1599: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4510, t4511, t4514, t4515, t4518, t4519, t4522, t4523, t4528) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1015::<F>(t344, t4509, t4338, t1409, t2989, t2988, t2987, t4343, t3966, t978, t977, t135, t1599);
    (t4510, t4511, t4514, t4515, t4518, t4519, t4522, t4523, t4528)
}
