//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2550;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2551;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2552;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2553;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2554;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2555;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2556;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta676<F: Float>(t1670: F, t3313: F, t11403: F, t3375: F, t4832: F, t11292: F, t1687: F, t50826: F, t43727: F, t43729: F, t43748: F, t43750: F, t50828: F, t50832: F, t50834: F, t50897: F, t50900: F, t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50919: F, t50921: F, t50926: F, t50931: F, t50934: F, t50948: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t43942: F, t50937: F, t50940: F, t50946: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F, t50994: F, t51000: F, t51004: F, t449: F, t11365: F, t1694: F, t3331: F, t4794: F, t1117: F, t14913: F, t3315: F, t11185: F, t14937: F, t3265: F, t4782: F, t11191: F, t11275: F, t4785: F, t44320: F, t11356: F, t11366: F, t11434: F, t1148: F, t1156: F, t15133: F, t3334: F, t3371: F, t3378: F, t436: F, t44211: F, t4802: F, t4858: F, t51107: F, t300: F, t51381: F, t51411: F, t51450: F, t51493: F, t51538: F, t51617: F, t51664: F, t15041: F, t3411: F, t11126: F, t4884: F, t1164: F, t44106: F, t4882: F, t14842: F, t11940: F, t4700: F, t5095: F, t51131: F, t51133: F, t51245: F, t51248: F, t51251: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51669, t51677, t51680, t51703) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2550::<F>(t1670, t3313, t11403, t3375, t4832, t11292, t1687, t50826, t43727, t43729, t43748, t43750, t50828, t50832, t50834, t50897, t50900, t50903, t50905, t50907, t50912, t50917, t50919, t50921, t50926, t50931, t50934);
        let t51723 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2551::<F>(t50948, t43780, t43782, t43784, t43786, t43788, t43816, t43942, t50937, t50940, t50946, t50950, t50952, t50954, t50957, t50961, t50966, t50994, t51000, t51004);
        let (t51725, t51727, t51730, t51736, t51738) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2552::<F>(t449, t51703, t51723, t11365, t1694, t3331, t4794, t1117, t14913, t3313, t3315, t11185, t14937);
        let (t51741, t51744, t51765) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2553::<F>(t3265, t3313, t4782, t11191, t11275, t4785, t50826, t50919, t43727, t43729, t43748, t43750, t50828, t50832, t50834, t50897, t50900, t50903, t50905, t50907, t50912, t50917, t50921, t50926, t50931, t50934);
        let t51785 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2554::<F>(t50948, t43780, t43782, t43784, t43786, t43788, t43816, t44320, t50937, t50940, t50946, t50950, t50952, t50954, t50957, t50961, t50966, t50994, t51000, t51004);
        let t51789 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2555::<F>(t11356, t11366, t11434, t1148, t1156, t15133, t3334, t3371, t3378, t436, t44211, t4802, t4858, t51107, t51669, t51677, t51680, t51725, t51727, t51730, t51736, t51738, t51741, t51744, t51765, t51785);
        let (t51793, t51795, t51797) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2556::<F>(t300, t51381, t51411, t51450, t51493, t51538, t51617, t51664, t51789, t15041, t3411, t11126, t4884);
        let (t51800, t51802, t51803) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2557::<F>(t1164, t44106, t4882, t14842, t3411, t11940, t4700, t5095, t51131, t51133, t51245, t51248, t51251, t51793, t51795, t51797);
    (t51669, t51725, t51736, t51738, t51741, t51744, t51793, t51795, t51797, t51800, t51802, t51803)
}
