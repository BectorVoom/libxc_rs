//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2375;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2376;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2377;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta647<F: Float>(t10662: F, t14395: F, t42028: F, t10829: F, t14258: F, t959: F, t10605: F, t4483: F, t10523: F, t2933: F, t4471: F, t47793: F, t47795: F, t47798: F, t47802: F, t48679: F, t48681: F, t48725: F, t48727: F, t48730: F, t48732: F, t48734: F, t48736: F, t48738: F, t48741: F, t48744: F, t48747: F, t14473: F, t2944: F, t10661: F, t1556: F, t10731: F, t14363: F, t300: F, t961: F, t2948: F, t14419: F, t923: F, t10771: F, t1568: F, t10756: F, t1580: F, t2930: F, t10717: F, t10720: F, t10744: F, t14271: F, t42671: F, t933: F, t950: F, t2885: F, t4408: F, t47705: F, t47707: F, t47730: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t47732: F, t47736: F, t47738: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48750, t48753, t48755, t48759, t48760) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2375::<F>(t10662, t14395, t42028, t10829, t14258, t959, t10605, t4483, t10523, t2933, t4471, t47793, t47795, t47798, t47802, t48679, t48681, t48725, t48727, t48730, t48732, t48734, t48736, t48738, t48741, t48744, t48747);
        let (t48762, t48765, t48768, t48770, t48771, t48776) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2376::<F>(t14473, t2944, t10661, t1556, t10731, t14363, t300, t961, t2948, t14419, t923, t10771, t1568);
        let t48786 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2377::<F>(t10756, t1580, t2930, t10717, t10720, t10744, t14271, t42671, t47798, t47802, t48725, t48730, t48732, t48734, t48736, t48738, t48741, t48744, t48771, t48776, t933, t950);
        let (t48789, t48813) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2378::<F>(t2885, t4408, t47705, t47707, t47730, t47681, t47686, t47691, t47695, t47699, t47703, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728, t47732, t47736, t47738);
    (t48750, t48753, t48755, t48759, t48760, t48762, t48765, t48768, t48770, t48786, t48789, t48813)
}
