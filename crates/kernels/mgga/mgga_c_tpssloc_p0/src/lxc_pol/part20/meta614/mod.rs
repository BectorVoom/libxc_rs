//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta614 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2204;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2205;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2206;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2207;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2208;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2209;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2210;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2211;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2212;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2213;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2214;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta614<F: Float>(t1454: F, t2585: F, t2281: F, t4044: F, t12758: F, t626: F, t12761: F, t12754: F, t4068: F, t12809: F, t92: F, t9384: F, t100: F, t9398: F, t2341: F, t657: F, t12774: F, t12775: F, t12778: F, t12795: F, t1447: F, t2219: F, t2248: F, t2336: F, t2342: F, t2350: F, t2354: F, t30171: F, t30307: F, t4049: F, t4050: F, t4054: F, t659: F, t662: F, t9212: F, t9393: F, t9404: F, t4063: F, t591: F, t4053: F, t103: F, t12771: F, t12781: F, t12784: F, t1444: F, t1445: F, t1449: F, t16: F, t2349: F, t4059: F, t45460: F, t45496: F, t584: F, t9374: F, t9385: F, t9399: F, t9400: F, t9407: F, t9408: F, t95: F, t12757: F, t12808: F, t1453: F, t2331: F, t2358: F, t26129: F, t29903: F, t45424: F, t45428: F, t45430: F, t45435: F, t64: F, t656: F, t666: F, t9366: F, t109: F, t2332: F, t4043: F, t4067: F, t45421: F, t45422: F, t45426: F, t45432: F, t9365: F, t9411: F, t25: F, t28: F, t88: F, t9416: F, t1406: F, t9238: F, t39031: F, zeta_threshold: F, t10913: F, t12595: F, t12598: F, t12606: F, t12609: F, t12612: F, t1409: F, t2244: F, t2250: F, t2291: F, t2298: F, t39096: F, t39114: F, t3966: F, t4007: F, t4012: F, t607: F, t634: F, t638: F, t9258: F, t9288: F, t9321: F, t9330: F, t12677: F, t12681: F, t12684: F, t12687: F, t1414: F, t1420: F, t2262: F, t39: F, t39210: F, t3982: F, t3985: F, t43: F, t51: F, t55: F, t615: F, t9277: F, t9301: F, t9308: F, t9287: F, t3961: F, t9300: F, t12680: F, t12698: F, t2267: F, t2274: F, t39159: F, t39168: F, t3981: F, t3990: F, t9305: F, t12620: F, t12630: F, t1427: F, t1434: F, t2245: F, t2284: F, t2304: F, t33: F, t3997: F, t3998: F, t4018: F, t629: F, t642: F, t66: F, t72: F, t80: F, t9251: F, t9313: F, t9339: F) -> (F, F, F, F, F, F, F) {
        let (t45656, t45659, t45660, t45662, t45676, t45689, t45690, t45697) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2204::<F>(t1454, t2585, t2281, t4044, t12758, t626, t12761, t12754, t4068, t12809, t92, t9384);
        let t45731 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2205::<F>(t100, t9398, t2341, t657, t12774, t12775, t12778, t12795, t1447, t2219, t2248, t2336, t2342, t2350, t2354, t30171, t30307, t4049, t4050, t4054, t45697, t659, t662, t92, t9212, t9393, t9404);
        let t45775 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2206::<F>(t100, t4063, t591, t4053, t92, t103, t12771, t12781, t12784, t1444, t1445, t1447, t1449, t16, t2341, t2349, t4059, t45460, t45496, t584, t657, t659, t662, t9374, t9385, t9399, t9400, t9407, t9408, t95);
        let t45780 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2207::<F>(t12757, t12808, t1453, t2331, t2358, t26129, t29903, t45424, t45428, t45430, t45435, t45676, t45689, t45690, t45731, t45775, t64, t656, t666, t9366);
        let t45782 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2208::<F>(t109, t2332, t4043, t4067, t45421, t45422, t45426, t45432, t45656, t45659, t45660, t45662, t45780, t64, t9365, t9411);
        let (t45814, t45844, t45872) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2209::<F>(t25, t28, t88, t9416, t1406, t9238, t16, t39031, zeta_threshold);
        let t45892 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2210::<F>(t10913, t12595, t12598, t12606, t12609, t12612, t1409, t2244, t2250, t2291, t2298, t39096, t39114, t3966, t4007, t4012, t45872, t607, t634, t638, t9258, t9288, t9321, t9330);
        let t45931 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2211::<F>(t12677, t12681, t12684, t12687, t1414, t1420, t2262, t39, t39210, t3982, t3985, t43, t45872, t51, t55, t615, t9277, t9301, t9308);
        let (t45970, t45971) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2212::<F>(t39, t9287, t2250, t3961);
        let t45977 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2213::<F>(t51, t9300, t12606, t12680, t12698, t1409, t1420, t2244, t2250, t2267, t2274, t39, t39159, t39168, t3966, t3981, t3990, t45970, t45971, t607, t9258, t9287, t9288, t9305);
        let t45986 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2214::<F>(t12620, t12630, t1427, t1434, t2244, t2245, t2284, t2304, t33, t3997, t3998, t4018, t45892, t45931, t45977, t629, t642, t66, t72, t80, t9251, t9313, t9339);
        let t45993 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2215::<F>(t1409, t9258);
    (t45782, t45814, t45844, t45872, t45971, t45986, t45993)
}
