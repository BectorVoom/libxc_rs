//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1523;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1524;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta391<F: Float>(t16040: F, t5250: F, t3851: F, t5335: F, t12248: F, t68: F, t544: F, t12250: F, t3791: F, t3793: F, t1332: F, t5333: F, t5230: F, t12240: F, t1352: F, t12189: F, t1804: F, t12188: F, t12190: F, t12194: F, t12196: F, t12197: F, t12200: F, t12205: F, t12209: F, t12212: F, t12228: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16041, t16044, t16047, t16048, t16049, t16052, t16055) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1523::<F>(t16040, t5250, t3851, t5335, t12248, t68, t544, t12250, t3791, t3793, t1332, t5333);
        let t16060 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1524::<F>(t5230, t68);
        let (t16065, t16068, t16080) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1525::<F>(t12240, t5335, t1352, t16040, t12189, t1804, t12188, t12190, t12194, t12196, t12197, t12200, t12205, t12209, t12212, t12228);
    (t16041, t16044, t16047, t16048, t16049, t16052, t16055, t16060, t16065, t16068, t16080)
}
