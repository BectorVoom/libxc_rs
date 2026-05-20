//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2029;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2030;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta462<F: Float>(t16048: F, t5335: F, t3793: F, t1332: F, t5333: F, t5230: F, t68: F, t12240: F, t1352: F, t16040: F, t12189: F, t1804: F, t12188: F, t12190: F, t12194: F, t12196: F, t12197: F, t12200: F, t12205: F, t12209: F, t12212: F, t12228: F, t5194: F, t782: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16049, t16052, t16055, t16060) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2029::<F>(t16048, t5335, t3793, t1332, t5333, t5230, t68);
        let (t16065, t16068, t16078, t16080) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2030::<F>(t12240, t5335, t1352, t16040, t12189, t1804, t12188, t12190, t12194, t12196, t12197, t12200, t12205, t12209, t12212, t12228);
        let t16081 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2031::<F>(t5194, t782);
    (t16049, t16052, t16055, t16060, t16065, t16068, t16078, t16080, t16081)
}
