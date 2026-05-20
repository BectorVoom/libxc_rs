//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta673 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2107;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta673<F: Float>(t24682: F, t460: F, t95413: F, t1409: F, t461: F, t1009: F, t7324: F, t24722: F, t15548: F, t24733: F, t27598: F, t3535: F, t2132: F, t24746: F, t3545: F, t8020: F, t1202: F, t27603: F, t24736: F, t4993: F, t15486: F, t7345: F, t27599: F, t3572: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t95415, t95420, t95422, t95424, t95435, t95440) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2107::<F>(t24682, t460, t95413, t1409, t461, t1009, t7324, t24722, t15548, t24733, t27598, t3535);
        let (t95446, t95450, t95452, t95456, t95459, t95463) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2108::<F>(t2132, t24746, t95413, t3545, t8020, t1202, t27603, t24736, t4993, t15486, t7345, t27599, t3572);
    (t95415, t95420, t95422, t95424, t95435, t95440, t95446, t95450, t95452, t95456, t95459, t95463)
}
