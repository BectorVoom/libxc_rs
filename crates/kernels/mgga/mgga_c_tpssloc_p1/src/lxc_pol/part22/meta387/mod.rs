//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta387 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta387<F: Float>(t1615: F, t4657: F, t1060: F, t1022: F, t360: F, t6739: F, t5928: F, t1049: F, t5866: F, t11066: F, t3201: F, t4649: F) -> (F, F, F, F, F, F, F, F) {
        let (t18089, t18093, t18094, t18099, t18100, t18103, t18104, t18107) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1655::<F>(t1615, t4657, t1060, t1022, t360, t6739, t5928, t1049, t5866, t11066, t3201, t4649);
    (t18089, t18093, t18094, t18099, t18100, t18103, t18104, t18107)
}
