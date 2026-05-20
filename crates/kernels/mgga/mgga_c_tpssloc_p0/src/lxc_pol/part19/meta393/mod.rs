//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta393 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1491;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1492;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1493;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1494;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1495;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta393<F: Float>(t626: F, t9412: F, t106: F, t9364: F, t2332: F, t2358: F, t2248: F, t35761: F, t2350: F, t2354: F, t39108: F, t35577: F, t2342: F, t100: F, t103: F, t2336: F, t2341: F, t2343: F, t2346: F, t2349: F, t657: F, t660: F, t92: F, t9276: F, t9374: F, t9384: F, t9386: F, t9389: F, t9390: F, t9393: F, t9394: F, t9398: F, t9403: F, t9407: F, t95: F, t96: F, tau0: F, t2331: F, t45421: F, t45422: F, t45424: F, t45426: F, t45428: F, t45430: F, t64: F, t656: F, t9365: F, t9370: F, t9411: F, t109: F, t11968: F, t11972: F, t12504: F, t12507: F, t1266: F, t1268: F, t12734: F, t2312: F, t2314: F, t2323: F, t2363: F, t2364: F, t3652: F, t39223: F, t39231: F, t39235: F, t4034: F, t45408: F, t510: F, t5113: F, t574: F, t650: F, t652: F, t671: F, t88: F, t9348: F, t9416: F, t45405: F, t112: F, t12512: F, t111: F, t3931: F, t12521: F, t12524: F, t12529: F, t12532: F, t1395: F, t1401: F, t16535: F, t2319: F, t3938: F, t3941: F, t577: F, t12513: F, t12537: F, t1396: F, t1398: F, t1404: F, t3: F, t39022: F, t39024: F, t39026: F, t39028: F, t3932: F, t3946: F, t580: F) -> F {
        let (t45432, t45435, t45436, t45444, t45453, t45460, t45461, t45469, t45482, t45496) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1491::<F>(t626, t9412, t106, t9364, t2332, t2358, t2248, t35761, t2350, t2354, t39108, t35577);
        let t45505 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1492::<F>(t2342, t100, t103, t2248, t2336, t2341, t2343, t2346, t2349, t2350, t2354, t45453, t45460, t45461, t45469, t45482, t45496, t657, t660, t92, t9276, t9374, t9384, t9386, t9389, t9390, t9393, t9394, t9398, t9403, t9407, t95, t96, tau0);
        let t45509 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1493::<F>(t2331, t2332, t2358, t45421, t45422, t45424, t45426, t45428, t45430, t45432, t45435, t45436, t45444, t45505, t64, t656, t9365, t9370, t9411);
        let (t45510, t45545) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1494::<F>(t109, t45509, t11968, t11972, t12504, t12507, t1266, t1268, t12734, t2312, t2314, t2323, t2363, t2364, t3652, t39223, t39231, t39235, t4034, t45408, t510, t5113, t574, t650, t652, t671, t88, t9348, t9416);
        let (t45546, t45580) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1495::<F>(t45405, t45545, t112, t12512, t111, t3931, t12521, t12524, t12529, t12532, t1395, t1401, t16535, t2319, t2363, t39231, t3938, t3941, t45510, t577, t671, t9416);
        let tv4rho40 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1496::<F>(t12513, t12537, t1396, t1398, t1404, t3, t39022, t39024, t39026, t39028, t3932, t3946, t45546, t45580, t580);
    tv4rho40
}
