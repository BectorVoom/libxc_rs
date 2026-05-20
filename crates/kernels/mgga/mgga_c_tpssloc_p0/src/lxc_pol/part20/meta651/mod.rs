//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta651 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2394;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2395;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2396;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2397;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2398;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta651<F: Float>(t13654: F, t2842: F, t2844: F, t912: F, t10727: F, t13727: F, t10731: F, t13520: F, t41811: F, t4359: F, t41623: F, t4400: F, t10817: F, t14379: F, t10655: F, t14389: F, t13655: F, t2792: F, t2904: F, t4446: F, t10523: F, t1573: F, t10629: F, t10750: F, t10757: F, t10820: F, t10829: F, t14337: F, t14344: F, t1581: F, t2900: F, t2907: F, t42106: F, t4472: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47705: F, t48085: F, t48087: F, t48090: F, t48092: F, t48096: F, t41831: F, t41833: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t47730: F, t41656: F, t41658: F, t41660: F, t47732: F, t47736: F, t47738: F, t47744: F, t47748: F, t48098: F, t48101: F, t48103: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49080, t49082, t49084, t49086, t49088) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2394::<F>(t13654, t2842, t2844, t912, t10727, t13727, t10731, t13520, t41811, t4359, t41623, t4400);
        let (t49090, t49092, t49095, t49096, t49099, t49104) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2395::<F>(t10817, t14379, t10655, t14389, t13655, t2792, t912, t2904, t4446, t10523, t1573, t10629);
        let t49113 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2396::<F>(t10750, t10757, t10820, t10829, t14337, t14344, t1581, t2900, t2907, t42106, t4472, t49080, t49082, t49084, t49086, t49088, t49090, t49092, t49095, t49096, t49099, t49104);
        let (t49127, t49139) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2397::<F>(t47681, t47686, t47691, t47695, t47699, t47703, t47705, t48085, t48087, t48090, t48092, t48096);
        let t49140 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2398::<F>(t41831, t41833, t47707, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728, t49139);
        let t49154 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2399::<F>(t47730, t41656, t41658, t41660, t47732, t47736, t47738, t47744, t47748, t48098, t48101, t48103);
    (t49080, t49082, t49084, t49086, t49088, t49090, t49092, t49095, t49113, t49127, t49140, t49154)
}
