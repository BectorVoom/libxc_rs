//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1862;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1863;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1864;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta485<F: Float>(t23664: F, t23720: F, t1055: F, t1065: F, t6815: F, t3174: F, t2780: F, t6690: F, t6689: F, t10170: F, t1052: F, t11010: F, t1956: F, t23579: F, t23582: F, t23589: F, t23595: F, t3026: F, t6680: F, t6687: F, t6700: F, t6816: F, t1923: F, t23310: F, t23314: F, t23317: F, t23323: F, t23327: F, t23333: F, t23337: F, t23341: F, t23346: F, t23381: F, t23574: F, t3169: F, t6707: F, t6776: F, t3216: F, t6818: F, t11094: F, t1958: F, t13487: F, t1877: F, t1915: F, t193: F, t202: F, t23285: F, t23290: F, t23295: F, t2379: F, t2522: F, t2553: F, t2745: F, t2749: F, t4314: F, t6666: F, t6670: F, t776: F, t868: F, t870: F, t265: F, t394: F, t1068: F, t1070: F, t3209: F, t3213: F, t336: F, t4700: F, t6822: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t23721, t23722, t23725, t23728, t23729, t23732) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1862::<F>(t23664, t23720, t1055, t1065, t6815, t3174, t2780, t6690, t6689, t10170, t1052, t11010, t1956, t23579, t23582, t23589, t23595, t3026, t6680, t6687, t6700, t6816);
        let t23734 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1863::<F>(t1052, t1923, t23310, t23314, t23317, t23323, t23327, t23333, t23337, t23341, t23346, t23381, t23574, t23732, t3026, t3169, t6687, t6707, t6776);
        let (t23738, t23742, t23772) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1864::<F>(t3216, t6818, t11094, t1958, t13487, t1877, t1915, t193, t202, t23285, t23290, t23295, t2379, t2522, t2553, t2745, t2749, t4314, t6666, t6670, t776, t868, t870);
        let t23773 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1865::<F>(t265, t394, t1068, t1070, t193, t23734, t23738, t23742, t23772, t3209, t3213, t336, t4700, t6822);
    (t23721, t23722, t23725, t23728, t23729, t23734, t23738, t23742, t23772, t23773)
}
