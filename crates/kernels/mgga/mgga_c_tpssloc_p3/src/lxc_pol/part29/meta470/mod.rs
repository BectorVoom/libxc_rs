//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1802;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1803;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1804;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta470<F: Float>(t2752: F, t6665: F, t10143: F, t1914: F, t25: F, t2749: F, t606: F, t868: F, t2745: F, t1877: F, t1915: F, t2249: F, t22951: F, t22959: F, t22961: F, t22964: F, t22968: F, t23286: F, t2522: F, t4314: F, t6542: F, t6666: F, t6670: F, t6671: F, t134: F, t221: F, t2250: F, t3: F, t3034: F, t371: F, t13487: F, t193: F, t202: F, t23285: F, t2379: F, t2553: F, t776: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t23290 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1802::<F>(t2752, t6665);
        let t23295 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1803::<F>(t10143, t1914);
        let (t23296, t23299, t23302, t23309) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1804::<F>(t25, t2749, t606, t868, t2745, t1877, t1915, t2249, t22951, t22959, t22961, t22964, t22968, t23286, t23290, t23295, t2522, t4314, t6542, t6666, t6670, t6671);
        let (t23383, t23413, t23508, t23598, t23772) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1805::<F>(t134, t221, t2250, t3, t3034, t371, t13487, t1877, t1915, t193, t202, t23285, t23290, t23295, t2379, t2522, t2553, t2745, t2749, t4314, t6666, t6670, t776, t868, t870);
    (t23290, t23295, t23296, t23299, t23302, t23309, t23383, t23413, t23508, t23598, t23772)
}
