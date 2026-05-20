//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta626<F: Float>(t16148: F, t24995: F, t8945: F, t22949: F, t7685: F, t25010: F, t6876: F, t1307: F, t19577: F, t8643: F, t1983: F, t22584: F, t26167: F) -> (F, F, F, F, F) {
        let (t86679, t86682, t86684, t86688, t86693) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2111::<F>(t16148, t24995, t8945, t22949, t7685, t25010, t6876, t1307, t19577, t8643, t1983, t22584, t26167);
    (t86679, t86682, t86684, t86688, t86693)
}
