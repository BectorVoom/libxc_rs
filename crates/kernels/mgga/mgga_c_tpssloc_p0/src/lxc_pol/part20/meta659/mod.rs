//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2452;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2453;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2454;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2455;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2456;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2457;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2458;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2459;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2460;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2461;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2462;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta659<F: Float>(t1020: F, t1616: F, t248: F, t43216: F, t10489: F, t4644: F, t10898: F, t4630: F, t10882: F, t48569: F, t10463: F, t10493: F, t10517: F, t10886: F, t10891: F, t10937: F, t10972: F, t13762: F, t14080: F, t14099: F, t1618: F, t3098: F, t42496: F, t42653: F, t43186: F, t4579: F, t4652: F, t13961: F, t3109: F, t10263: F, t10321: F, t10403: F, t10408: F, t14122: F, t3070: F, t3071: F, t3132: F, t42505: F, t42541: F, t43200: F, t43206: F, t43214: F, t43219: F, t43221: F, t43226: F, t43241: F, t4337: F, t4347: F, t4609: F, t13542: F, t2970: F, t973: F, t13546: F, t10480: F, t13969: F, t13986: F, t3039: F, t4599: F, t49850: F, t10870: F, t10875: F, t10879: F, t10904: F, t13977: F, t13987: F, t14001: F, t14006: F, t2960: F, t42561: F, t43228: F, t43233: F, t47701: F, t977: F, t13765: F, t3040: F, t607: F, t883: F, t1023: F, t10957: F, t10962: F, t14211: F, t14215: F, t42388: F, t42483: F, t43246: F, t43248: F, t43253: F, t43254: F, t43361: F, t4585: F, t4590: F, t48611: F, t48612: F, t49616: F, t49976: F, t10903: F, t14507: F, t14651: F, t3069: F, t10394: F, t10952: F, t13995: F, t14069: F, t14077: F, t14103: F, t1622: F, t3073: F, t3120: F, t3123: F, t3134: F, t42746: F, t43262: F, t43273: F, t43277: F, t43281: F, t43285: F, t10956: F, t1611: F, t10459: F, t4608: F, t698: F, t10398: F, t1041: F, t10419: F, t1044: F, t1046: F, t14085: F, t14147: F, t14187: F, t14189: F, t3057: F, t3117: F, t43301: F, t4582: F, t4588: F, t45997: F, t47734: F, t48554: F, t2770: F, t2987: F, t10277: F, t4509: F, t10390: F, t14501: F, t10915: F, t13554: F, t14033: F, t14037: F, t2986: F, t43303: F, t43307: F, t43310: F, t43313: F, t4575: F, t45971: F, t10305: F, t10857: F, t10858: F, t14041: F, t1539: F, t3121: F, t3130: F, t3131: F, t42397: F, t43325: F, t43332: F, t43336: F, t43341: F, t43350: F, t43352: F, t43354: F, t4593: F, t1606: F, t2402: F, t10454: F, t13950: F, t14202: F, t3048: F, t14206: F, t3108: F, t1025: F, t10501: F, t3064: F, t43374: F, t43377: F, t43382: F, t43406: F, t43410: F, t47775: F, t48497: F, t48471: F, t48511: F, t48543: F, t48577: F, t48622: F, t48656: F, t49609: F, t49654: F, t49688: F, t49718: F, t49750: F, t49786: F, t49824: F, t49860: F, t49891: F, t49924: F, t49961: F, t49991: F, t50035: F, t50066: F, t50102: F, t50136: F, t50176: F, t3185: F, t49649: F, t11031: F, t11054: F, t11081: F, t14578: F, t14596: F, t14605: F, t14608: F, t14622: F, t1629: F, t1630: F, t3076: F, t3180: F, t3186: F, t3189: F, t3200: F, t43473: F, t43515: F, t43542: F, t4669: F, t4680: F, t4684: F, t4691: F, t47819: F) -> (F, F) {
        let t50207 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2452::<F>(t1020, t1616, t248, t43216, t10489, t4644, t10898, t4630, t10882, t48569, t10463, t10493, t10517, t10886, t10891, t10937, t10972, t13762, t14080, t14099, t1618, t3098, t42496, t42653, t43186, t4579, t4652);
        let t50237 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2453::<F>(t13961, t3109, t10263, t10321, t10403, t10408, t14122, t1616, t3070, t3071, t3132, t42505, t42541, t43200, t43206, t43214, t43219, t43221, t43226, t43241, t4337, t4347, t4609);
        let (t50242, t50250, t50255, t50259, t50262) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2454::<F>(t13542, t2970, t973, t13546, t10480, t13969, t13986, t3039, t4599, t49850, t10870, t4644);
        let t50268 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2455::<F>(t50262, t10875, t48569, t10879, t10904, t13977, t13987, t14001, t14006, t2960, t42561, t43228, t43233, t47701, t50242, t50250, t50255, t50259, t973, t977);
        let t50301 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2456::<F>(t10937, t13765, t3040, t607, t883, t1023, t10957, t10962, t14211, t14215, t3070, t3071, t42388, t42483, t42505, t43246, t43248, t43253, t43254, t43361, t4337, t4585, t4590, t4652, t48611, t48612, t49616, t49976);
        let t50329 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2457::<F>(t10903, t14507, t14651, t3069, t10394, t10403, t10937, t10952, t13995, t14069, t14077, t14103, t14211, t1622, t3071, t3073, t3120, t3123, t3134, t42746, t43262, t43273, t43277, t43281, t43285, t607, t883);
        let t50365 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2458::<F>(t10956, t1611, t10517, t4630, t10459, t4644, t4608, t698, t973, t10398, t1041, t10419, t1044, t1046, t13995, t14085, t14147, t14187, t14189, t248, t3057, t3117, t43301, t4582, t4588, t45997, t47734, t48554);
        let t50393 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2459::<F>(t2770, t2987, t10277, t4509, t10390, t13765, t10937, t14501, t10408, t10915, t13554, t14033, t14037, t2986, t3070, t42496, t43303, t43307, t43310, t43313, t4575, t45971, t4644, t49976);
        let t50423 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2460::<F>(t10305, t10390, t10857, t10858, t10891, t14041, t14103, t1539, t1616, t3070, t3071, t3121, t3130, t3131, t42397, t43325, t43332, t43336, t43341, t43350, t43352, t43354, t4347, t4582, t4593);
        let t50452 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2461::<F>(t1606, t2402, t973, t10454, t4644, t13950, t3117, t14202, t3048, t14206, t3108, t1025, t1041, t10501, t14085, t1622, t3064, t3098, t43374, t43377, t43382, t43406, t43410, t4582, t47775, t48497);
        let t50457 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2462::<F>(t48471, t48511, t48543, t48577, t48622, t48656, t49609, t49654, t49688, t49718, t49750, t49786, t49824, t49860, t49891, t49924, t49961, t49991, t50035, t50066, t50102, t50136, t50176, t50207, t50237, t50268, t50301, t50329, t50365, t50393, t50423, t50452);
        let t50490 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2463::<F>(t3185, t49649, t11031, t11054, t11081, t14578, t14596, t14605, t14608, t14622, t1629, t1630, t3076, t3131, t3180, t3186, t3189, t3200, t43473, t43515, t43542, t4669, t4680, t4684, t4691, t47819);
    (t50457, t50490)
}
