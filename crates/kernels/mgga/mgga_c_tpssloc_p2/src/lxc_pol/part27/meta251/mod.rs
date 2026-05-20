//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1218;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1219;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1220;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1221;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1222;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta251<F: Float>(t1059: F, t6800: F, t6799: F, t1049: F, t1948: F, t345: F, t1022: F, t1945: F, t1060: F, t383: F, t6768: F, t1003: F, t1058: F, t1920: F, t1950: F, t1953: F, t353: F, t6680: F, t6687: F, t6783: F, t6787: F, t6790: F, t6797: F, t1055: F, t1052: F, t1066: F, t1923: F, t1956: F, t3026: F, t3169: F, t388: F, t6685: F, t6692: F, t6695: F, t6700: F, t6707: F, t6710: F, t6769: F, t6771: F, t6776: F, t1958: F, t3216: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6801, t6802) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1218::<F>(t1059, t6800, t6799);
        let t6805 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1219::<F>(t1049, t1948);
        let (t6806, t6811, t6813, t6815) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1220::<F>(t345, t6805, t1022, t1945, t1060, t383, t6768, t1003, t1058, t1920, t1950, t1953, t353, t6680, t6687, t6783, t6787, t6790, t6797, t6802);
        let t6816 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1221::<F>(t1055, t6815);
        let t6818 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1222::<F>(t1052, t1066, t1920, t1923, t1956, t3026, t3169, t388, t6680, t6685, t6687, t6692, t6695, t6700, t6707, t6710, t6769, t6771, t6776, t6816);
        let t6822 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1223::<F>(t1958, t3216);
    (t6801, t6802, t6805, t6806, t6811, t6813, t6815, t6816, t6818, t6822)
}
