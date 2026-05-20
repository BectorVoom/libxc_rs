//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2210;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2211;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2212;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta708<F: Float>(t5544: F, t606: F, t16662: F, t25: F, t2752: F, t28447: F, t28248: F, t776: F, t22960: F, t10143: F, t1408: F, t25374: F, t1530: F, t584: F, t86730: F, t25372: F, t5397: F, t868: F, t81547: F, t5660: F, t17109: F, t1877: F, t1915: F, t22959: F, t23290: F, t25013: F, t2522: F, t28249: F, t28448: F, t28459: F, t6666: F, t6670: F, t6671: F, t81483: F, t5664: F, t25373: F, t23168: F, t28288: F, t10109: F, t1888: F, t23270: F, t5636: F, t865: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98046, t98050, t98054, t98058, t98059, t98065) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2210::<F>(t5544, t606, t16662, t25, t2752, t28447, t28248, t776, t22960, t10143, t1408, t25374);
        let (t98071, t98075, t98079, t98082, t98086) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2211::<F>(t1530, t584, t86730, t25372, t5397, t868, t28248, t81547, t5660, t606, t17109, t25);
        let t98090 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2212::<F>(t1877, t1915, t22959, t23290, t25013, t2522, t25372, t28249, t28448, t28459, t5397, t606, t6666, t6670, t6671, t81483, t98046, t98050, t98054, t98059, t98065, t98071, t98075, t98079, t98082, t98086);
        let (t98091, t98094, t98102, t98103, t98111, t98112, t98117, t98122) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2213::<F>(t5664, t606, t5397, t776, t5660, t868, t25373, t28248, t23168, t28288, t10109, t1888, t23270, t5636, t865);
    (t98054, t98058, t98071, t98090, t98091, t98094, t98102, t98103, t98111, t98112, t98117, t98122)
}
