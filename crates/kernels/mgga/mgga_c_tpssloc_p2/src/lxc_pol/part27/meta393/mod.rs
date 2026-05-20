//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1606;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1607;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1608;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1609;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1610;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1611;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta393<F: Float>(t11285: F, t3377: F, t14853: F, t1164: F, t300: F, t4832: F, t1166: F, t3419: F, t4869: F, t11180: F, t1671: F, t3259: F, t4782: F, t14704: F, t14710: F, t14722: F, t11215: F, t11217: F, t14720: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t14766: F, t14781: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F, t14814: F, t14816: F, t14818: F, t14824: F, t11195: F, t11204: F, t11211: F, t11213: F, t14702: F, t14708: F, t14713: F, t14759: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t14802: F, t14805: F, t1118: F, t1099: F, t11136: F, t449: F, t3265: F, t3313: F, t11459: F, t423: F, t1254: F, t14696: F, t14701: F, t14833: F, t14835: F, t14837: F, t14840: F, t14844: F, t14847: F, t14849: F, t14852: F, t4700: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14857, t14860, t14862, t14864, t14866) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1606::<F>(t11285, t3377, t14853, t1164, t300, t4832, t1166, t3419, t4869, t11180, t1671, t3259, t4782);
        let (t14868, t14870, t14887) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1607::<F>(t14704, t14710, t14722, t11215, t11217, t14720, t14733, t14738, t14742, t14746, t14751, t14755, t14766);
        let (t14890, t14911) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1608::<F>(t14781, t11137, t11139, t11141, t11143, t14728, t14809, t14811, t14814, t14816, t14818, t14824);
        let t14913 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1609::<F>(t11195, t11204, t11211, t11213, t14702, t14708, t14713, t14759, t14779, t14784, t14787, t14790, t14793, t14796, t14799, t14802, t14805, t14868, t14870, t14887, t14890, t14911);
        let (t14916, t14933) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1610::<F>(t1118, t14913, t1099, t14720, t14722, t14704, t11136, t11137, t11139, t11141, t11143, t14702, t14708, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
        let (t14934, t14936, t14939, t14956) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1611::<F>(t14933, t449, t300, t1671, t3265, t3313, t14722, t14704, t11137, t11139, t11141, t11143, t11459, t14702, t14708, t14720, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
        let (t14958, t14959) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1612::<F>(t14956, t423, t1254, t14696, t14701, t14833, t14835, t14837, t14840, t14844, t14847, t14849, t14852, t14857, t14860, t14862, t14864, t14866, t14916, t14936, t14939, t4700);
    (t14857, t14860, t14862, t14864, t14866, t14916, t14934, t14936, t14939, t14958, t14959)
}
