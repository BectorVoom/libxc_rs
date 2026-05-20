//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta745 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2473;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2474;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2475;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2476;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2477;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2478;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2479;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta745<F: Float>(t1020: F, t21595: F, t248: F, t3101: F, t14511: F, t17655: F, t10883: F, t21403: F, t1041: F, t21130: F, t42592: F, t21594: F, t376: F, t1023: F, t10413: F, t14077: F, t21516: F, t21532: F, t3039: F, t3048: F, t3070: F, t3071: F, t42483: F, t42546: F, t4347: F, t4582: F, t48611: F, t48670: F, t48674: F, t5681: F, t5867: F, t5869: F, t61866: F, t70086: F, t70122: F, t10422: F, t21519: F, t10403: F, t10408: F, t10904: F, t21487: F, t49662: F, t5677: F, t61916: F, t61919: F, t61923: F, t61929: F, t61940: F, t61975: F, t61977: F, t70082: F, t1616: F, t17187: F, t17980: F, t42552: F, t4575: F, t4650: F, t49691: F, t49693: F, t50193: F, t61950: F, t61981: F, t62013: F, t62032: F, t62038: F, t3966: F, t20217: F, t607: F, t10949: F, t14211: F, t21538: F, t21562: F, t2960: F, t3130: F, t4588: F, t4596: F, t4600: F, t61736: F, t61739: F, t62091: F, t62137: F, t62148: F, t62150: F, t62152: F, t135: F, t21561: F, t973: F, t10390: F, t14207: F, t17712: F, t17732: F, t17984: F, t21526: F, t21566: F, t369: F, t378: F, t42505: F, t50265: F, t5878: F, t62164: F, t62177: F, t62183: F, t68: F, t70012: F, t21525: F, t10876: F, t14508: F, t1539: F, t17670: F, t17714: F, t17890: F, t17960: F, t21118: F, t21398: F, t21512: F, t42565: F, t4644: F, t47779: F, t62210: F, t62234: F, t70330: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t70346, t70351, t70363, t70389, t70391) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2473::<F>(t1020, t21595, t248, t3101, t14511, t17655, t10883, t21403, t1041, t21130, t42592, t21594, t376);
        let t70396 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2474::<F>(t1023, t10413, t14077, t21516, t21532, t3039, t3048, t3070, t3071, t42483, t42546, t4347, t4582, t48611, t48670, t48674, t5681, t5867, t5869, t61866, t70086, t70122, t70389, t70391);
        let t70414 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2475::<F>(t10422, t21519, t3070, t10403, t10408, t10904, t21487, t49662, t5677, t61916, t61919, t61923, t61929, t61940, t61975, t61977, t70082);
        let t70432 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2476::<F>(t10408, t1616, t17187, t17980, t3070, t3071, t42552, t4575, t4650, t49691, t49693, t50193, t5677, t61950, t61981, t62013, t62032, t62038);
        let (t70442, t70458) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2477::<F>(t1616, t3966, t20217, t607);
        let t70481 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2478::<F>(t1041, t10949, t14211, t21487, t21538, t21562, t2960, t3130, t4582, t4588, t4596, t4600, t61736, t61739, t62091, t62137, t62148, t62150, t62152, t70458);
        let t70509 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2479::<F>(t135, t21561, t973, t10390, t10413, t14207, t17712, t17732, t17984, t21526, t21566, t3071, t3130, t369, t378, t42505, t4347, t4582, t50265, t5869, t5878, t62164, t62177, t62183, t68, t70012);
        let t70539 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2480::<F>(t10403, t10422, t21525, t1023, t10408, t1041, t10876, t14508, t1539, t17670, t17714, t17732, t17890, t17960, t21118, t21398, t21512, t3048, t3070, t3071, t42565, t4582, t4644, t47779, t62210, t62234, t70330);
    (t70346, t70351, t70363, t70391, t70396, t70414, t70432, t70442, t70458, t70481, t70509, t70539)
}
