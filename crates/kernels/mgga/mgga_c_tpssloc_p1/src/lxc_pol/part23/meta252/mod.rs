//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk912;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta252<F: Float>(t248: F, t3521: F, t5979: F, t1227: F, t1009: F, t6150: F, t1011: F, t1212: F, t1226: F, t6169: F, t486: F, t6218: F, t5001: F, t5018: F, t1730: F, t5023: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19040, t19041, t19045, t19046, t19047, t19051, t19056) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk912::<F>(t248, t3521, t5979, t1227, t1009, t6150, t1011, t1212, t1226, t6169, t486, t6218);
        let (t19080, t19083) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk913::<F>(t5001, t5018, t1730, t5023);
    (t19040, t19041, t19045, t19046, t19047, t19051, t19056, t19080, t19083)
}
