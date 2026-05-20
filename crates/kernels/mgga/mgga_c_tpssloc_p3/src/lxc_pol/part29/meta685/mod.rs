//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta685 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2338;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2339;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2340;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2341;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2342;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2343;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2344;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2345;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2346;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2347;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2348;
use chunk11::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta685<F: Float>(t27331: F, t9231: F, t2110: F, t22519: F, t22531: F, t22537: F, t24511: F, t24526: F, t26090: F, t27332: F, t6492: F, t7246: F, t7432: F, t7435: F, t7975: F, t7978: F, t85514: F, t85524: F, t90297: F, t90337: F, t90340: F, t24505: F, t24508: F, t26070: F, t26073: F, t26076: F, t7256: F, t7259: F, t90150: F, t90153: F, t90160: F, t90343: F, t46104: F, t7245: F, t12571: F, t24525: F, t9239: F, t22527: F, t22546: F, t24514: F, t26055: F, t27341: F, t85510: F, t90196: F, t90202: F, t90205: F, t2240: F, t27363: F, t33: F, t24520: F, t26063: F, t26067: F, t27308: F, t27311: F, t27365: F, t6495: F, t90177: F, t90227: F, t90232: F, t90334: F, t26012: F, t7255: F, t22549: F, t24517: F, t26009: F, t27298: F, t27303: F, t83722: F, t83778: F, t85463: F, t85480: F, t85501: F, t85536: F, t90080: F, t90114: F, t90137: F, t90141: F, t2109: F, t90090: F, t90094: F, t45844: F, t26016: F, t85470: F, t85473: F, t85476: F, t85507: F, t90072: F, t90076: F, t90098: F, t90101: F, t90104: F, t22550: F, t7974: F, t90247: F, t1419: F, t2274: F, t12606: F, t12648: F, t12652: F, t14165: F, t1860: F, t1864: F, t2108: F, t2244: F, t2250: F, t24498: F, t26028: F, t27356: F, t27364: F, t608: F, t6486: F, t6509: F, t67: F, t7251: F, t7428: F, t83803: F, t85539: F, t90121: F, t22489: F, t22493: F, t22534: F, t24504: F, t26024: F, t7445: F, t90132: F, t90257: F, t5: F, t112: F, t671: F, t7263: F, t12813: F, t1459: F, t1849: F, t2165: F, t2314: F, t24932: F, t24939: F, t27293: F, t3929: F, t4037: F, t510: F, t652: F, t8107: F, t91666: F, t91671: F, t91673: F, t91674: F, t91678: F, t91681: F, t91684: F, t91690: F, t91694: F, t91698: F, t91704: F, t91706: F, t2320: F, t8103: F, t91708: F, t91713: F, t91715: F, t91718: F, t91722: F, t91724: F, t91726: F, t91730: F, t91735: F, t91737: F, t91739: F, t91747: F, t91749: F, t91752: F, t91755: F, t91757: F, t91759: F, t91762: F, t111: F, t27370: F, t1458: F, t2363: F, t27863: F, t27888: F, t4072: F, t7266: F, t85428: F, t90355: F, t90361: F, t90363: F, t90365: F, t90367: F, t90369: F, t94248: F, t90372: F, t90374: F, t90377: F, t90379: F, t90383: F, t90385: F, t90387: F, t90399: F, t90404: F, t90406: F, t90408: F, t90410: F, t94265: F) -> (F, F, F, F, F) {
        let t95996 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2338::<F>(t27331, t9231, t2110, t22519, t22531, t22537, t24511, t24526, t26090, t27332, t6492, t7246, t7432, t7435, t7975, t7978, t85514, t85524, t90297, t90337, t90340);
        let t96021 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2339::<F>(t2110, t24505, t24508, t26070, t26073, t26076, t7256, t7259, t7435, t90150, t90153, t90160, t90343);
        let t96050 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2340::<F>(t46104, t7245, t12571, t24525, t27331, t9239, t2110, t22527, t22531, t22537, t22546, t24514, t26055, t27341, t6492, t7256, t7259, t7432, t7978, t85510, t90196, t90202, t90205);
        let t96077 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2341::<F>(t2240, t27363, t33, t24520, t24526, t26063, t26067, t27308, t27311, t27365, t6492, t6495, t7246, t90177, t90227, t90232, t90334);
        let t96105 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2342::<F>(t26012, t7255, t22527, t22549, t24514, t24517, t24520, t26009, t26090, t27298, t27303, t27332, t6495, t83722, t83778, t85463, t85480, t85501, t85536, t90080, t90114, t90137, t90141);
        let t96133 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2343::<F>(t2109, t90090, t90094, t45844, t7245, t22546, t22549, t24514, t24517, t26016, t7432, t85470, t85473, t85476, t85507, t90072, t90076, t90098, t90101, t90104);
        let t96180 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2344::<F>(t22550, t7974, t2109, t90247, t1419, t2274, t12606, t12648, t12652, t14165, t1860, t1864, t2108, t2110, t2244, t2250, t22549, t24498, t24505, t24508, t26009, t26028, t27303, t27356, t27364, t27365, t608, t6486, t6509, t67, t7251, t7256, t7259, t7428, t83803, t85539, t90121, t9239);
        let t96209 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2345::<F>(t1860, t2109, t2110, t22489, t22493, t22534, t24504, t24511, t26024, t27308, t27311, t6486, t7255, t7428, t7445, t7974, t7975, t7978, t90132, t90257);
        let (t96214, t96222, t96228) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2346::<F>(t5, t95996, t96021, t96050, t96077, t96105, t96133, t96180, t96209, t112, t671, t7263, t12813, t1459, t1849, t2165, t2314, t24932, t24939, t27293, t3929, t4037, t510, t652, t8107, t91666, t91671, t91673, t91674, t91678, t91681, t91684, t91690, t91694, t91698, t91704, t91706);
        let t96232 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2347::<F>(t2320, t8103, t91708, t91713, t91715, t91718, t91722, t91724, t91726, t91730, t91735, t91737, t91739, t91747, t91749, t91752, t91755, t91757, t91759, t91762);
        let (t96238, t96269) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2348::<F>(t111, t27370, t12813, t1458, t2363, t24932, t27863, t27888, t4072, t671, t7266, t85428, t90355, t90361, t90363, t90365, t90367, t90369, t94248, t96222);
        let t96271 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2349::<F>(t90372, t90374, t90377, t90379, t90383, t90385, t90387, t90399, t90404, t90406, t90408, t90410, t94265, t96214);
    (t96228, t96232, t96238, t96269, t96271)
}
