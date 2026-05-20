//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta632<F: Float>(t22949: F, t7685: F, t25010: F, t6876: F, t1307: F, t19577: F, t24995: F, t8643: F, t1983: F, t22584: F, t26167: F, t12725: F, t6535: F) -> (F, F, F, F, F) {
        let (t86682, t86684, t86688, t86693, t86698) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2080::<F>(t22949, t7685, t25010, t6876, t1307, t19577, t24995, t8643, t1983, t22584, t26167, t12725, t6535);
    (t86682, t86684, t86688, t86693, t86698)
}
