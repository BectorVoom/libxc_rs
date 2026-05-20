//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta520<F: Float>(t6722: F, t7573: F, t3: F, t3966: F, t1933: F, t4603: F, t6717: F, t1597: F, t1934: F, t1025: F, t1046: F, t1607: F, t1618: F, t1920: F, t1937: F, t23419: F, t23422: F, t23425: F, t23437: F, t25571: F, t25574: F, t25577: F, t25580: F, t4575: F, t4579: F, t6735: F) -> (F, F, F, F, F, F) {
        let (t25585, t25588, t25589, t25600, t25601, t25605) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1925::<F>(t6722, t7573, t3, t3966, t1933, t4603, t6717, t1597, t1934, t1025, t1046, t1607, t1618, t1920, t1937, t23419, t23422, t23425, t23437, t25571, t25574, t25577, t25580, t4575, t4579, t6735);
    (t25585, t25588, t25589, t25600, t25601, t25605)
}
