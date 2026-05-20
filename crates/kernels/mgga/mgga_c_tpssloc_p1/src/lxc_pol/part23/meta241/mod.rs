//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta241<F: Float>(t17808: F, t2986: F, t10254: F, t5392: F, t135: F, t5844: F, t973: F, t5838: F, t10236: F, t10457: F, t248: F, t5677: F) -> (F, F, F, F, F, F) {
        let (t17809, t17817, t17827, t17850, t17863, t17884) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk897::<F>(t17808, t2986, t10254, t5392, t135, t5844, t973, t5838, t10236, t10457, t248, t5677);
    (t17809, t17817, t17827, t17850, t17863, t17884)
}
