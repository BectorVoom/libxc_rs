//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta689 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2133;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2134;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta689<F: Float>(t26168: F, t7685: F, t19924: F, t24995: F, t8945: F, t19456: F, t7468: F, t26003: F, t4028: F, t2314: F, t28864: F, t4034: F, t1873: F, t19289: F, t652: F, t1983: F, t20085: F, t6996: F, t28827: F, t6876: F, t7684: F, t8944: F, t26164: F, t75203: F, t8643: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t96760, t96763, t96765, t96767, t96784, t96786) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2133::<F>(t26168, t7685, t19924, t24995, t8945, t19456, t7468, t26003, t4028, t2314, t28864, t4034);
        let (t96789, t96792, t96796, t96799, t96802) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2134::<F>(t1873, t19289, t652, t1983, t20085, t6996, t28827, t6876, t7684, t8944, t26164, t24995, t75203, t8643);
    (t96760, t96763, t96765, t96767, t96784, t96786, t96789, t96792, t96796, t96799, t96802)
}
