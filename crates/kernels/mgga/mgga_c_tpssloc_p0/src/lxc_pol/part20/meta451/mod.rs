//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1901;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1902;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1903;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta451<F: Float>(t15067: F, t3265: F, t11275: F, t14704: F, t14710: F, t14720: F, t11215: F, t11217: F, t14722: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t14766: F, t14781: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F, t14814: F, t14816: F, t14818: F, t14824: F, t11211: F, t11213: F, t11314: F, t11317: F, t14702: F, t14708: F, t14713: F, t14759: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t14802: F, t14805: F, t1137: F, t1147: F, t4832: F, t1687: F, t3400: F, t1156: F, t14829: F, t3375: F, t1129: F, t11356: F, t1148: F, t1157: F, t14840: F, t14847: F, t14849: F, t14852: F, t1695: F, t3371: F, t3378: F, t3396: F, t3404: F, t4835: F, t4858: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15068, t15070, t15072, t15074, t15091) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1901::<F>(t15067, t3265, t11275, t14704, t14710, t14720, t11215, t11217, t14722, t14733, t14738, t14742, t14746, t14751, t14755, t14766);
        let (t15094, t15115) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1902::<F>(t14781, t11137, t11139, t11141, t11143, t14728, t14809, t14811, t14814, t14816, t14818, t14824);
        let t15117 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1903::<F>(t11211, t11213, t11314, t11317, t14702, t14708, t14713, t14759, t14779, t14784, t14787, t14790, t14793, t14796, t14799, t14802, t14805, t15072, t15074, t15091, t15094, t15115);
        let (t15118, t15121, t15126, t15133, t15136, t15139) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1904::<F>(t1137, t15117, t1147, t4832, t1687, t3400, t1156, t14829, t3375, t1129, t11356, t1148, t1157, t14840, t14847, t14849, t14852, t1695, t3371, t3378, t3396, t3404, t4835, t4858);
    (t15068, t15070, t15117, t15118, t15121, t15126, t15133, t15136, t15139)
}
