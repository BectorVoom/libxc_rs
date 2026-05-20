//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1747;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1748;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1749;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta463<F: Float>(t225: F, t6625: F, t6576: F, t2752: F, t6665: F, t10143: F, t1914: F, t134: F, t221: F, t3034: F, t371: F, t28: F, t2274: F, t50: F, t7245: F, t9239: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t23278, t23281, t23290) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1747::<F>(t225, t6625, t6576, t2752, t6665);
        let t23295 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1748::<F>(t10143, t1914);
        let (t23383, t23508, t23598, t23788) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1749::<F>(t134, t221, t3034, t371, t2752, t28);
        let (t24498, t24514) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1750::<F>(t2274, t50, t7245, t9239);
    (t23278, t23281, t23290, t23295, t23383, t23508, t23598, t23788, t24498, t24514)
}
