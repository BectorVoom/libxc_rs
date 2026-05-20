//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta668 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2355;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2356;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2357;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2358;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2359;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2360;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2361;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta668<F: Float>(t12734: F, t7461: F, t2314: F, t25980: F, t22574: F, t56120: F, t8643: F, t1845: F, t3719: F, t1874: F, t55962: F, t19456: F, t6525: F, t22480: F, t4028: F, t12545: F, t1774: F, t22461: F, t22600: F, t2364: F, t24999: F, t25965: F, t4077: F, t6517: F, t7472: F, t91578: F, t91580: F, t91582: F, t91585: F, t91587: F, t91589: F, t26502: F, t532: F, t1983: F, t6879: F, t26142: F, t4034: F, t1266: F, t26135: F, t652: F, t24987: F, t6997: F, t22591: F, t24990: F, t6880: F, t22573: F, t7684: F, t22575: F, t22585: F, t7685: F, t12725: F, t12813: F, t1976: F, t22483: F, t2312: F, t2323: F, t24983: F, t25958: F, t3652: F, t4026: F, t650: F, t6539: F, t671: F, t6862: F, t7451: F, t7670: F, t22607: F, t7754: F, t6875: F, t8944: F, t26164: F, t22578: F, t7753: F, t7756: F, t531: F, t7752: F, t22596: F, t16153: F, t24995: F, t8945: F, t25988: F, t31035: F, t2018: F, t40611: F, t3698: F, t26161: F, t15868: F, t6996: F, t3734: F, t23831: F, t7458: F, t9348: F, t12724: F, t12823: F, t12835: F, t24980: F, t3929: F, t7681: F, t22479: F, t7468: F, t15904: F, t33136: F, t26003: F, t90381: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91591, t91593, t91602, t91606, t91608, t91610) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2355::<F>(t12734, t7461, t2314, t25980, t22574, t56120, t8643, t1845, t3719, t1874, t55962, t19456, t6525);
        let t91617 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2356::<F>(t22480, t4028, t12545, t12734, t1774, t22461, t22600, t2314, t2364, t24999, t25965, t4077, t6517, t7472, t91578, t91580, t91582, t91585, t91587, t91589, t91591, t91593, t91602, t91606, t91608, t91610);
        let (t91623, t91625, t91627, t91630, t91637) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2357::<F>(t26502, t532, t1983, t6879, t2314, t26142, t4034, t1266, t26135, t652, t24987, t6997);
        let t91663 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2358::<F>(t1983, t22591, t24990, t24987, t6880, t22573, t7684, t22575, t22585, t7685, t12725, t12813, t1976, t22483, t2312, t2314, t2323, t24983, t24999, t25958, t3652, t4026, t4028, t650, t652, t6539, t671, t6862, t7451, t7670, t91623, t91625, t91627, t91630, t91637);
        let (t91666, t91671, t91673, t91674, t91678) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2359::<F>(t22607, t7754, t6875, t8944, t26164, t1983, t22578, t7753, t7756, t531, t7752, t22596);
        let (t91681, t91684, t91690, t91694) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2360::<F>(t16153, t24995, t8945, t22574, t25988, t31035, t2018, t40611, t1845, t3698, t26161, t15868, t1983, t6996);
        let t91709 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2361::<F>(t1845, t3734, t24995, t8643, t23831, t7458, t22480, t7461, t9348, t12724, t12823, t12835, t1976, t2314, t24980, t25965, t3929, t4034, t6517, t7472, t7681, t91666, t91671, t91673, t91674, t91678, t91681, t91684, t91690, t91694);
        let (t91713, t91715, t91718, t91722, t91724, t91726) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2362::<F>(t1774, t22479, t652, t7468, t9348, t15904, t22574, t33136, t12734, t2314, t26003, t1874, t90381);
    (t91617, t91663, t91709, t91713, t91715, t91718, t91722, t91724, t91726)
}
