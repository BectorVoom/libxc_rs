//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta686 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2350;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2351;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2352;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2353;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2354;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2355;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta686<F: Float>(t12823: F, t15857: F, t2114: F, t2312: F, t2314: F, t2323: F, t27290: F, t27858: F, t27863: F, t27879: F, t4034: F, t5107: F, t5361: F, t574: F, t652: F, t671: F, t672: F, t7264: F, t7412: F, t7989: F, t8103: F, t91763: F, t91765: F, t91767: F, t91769: F, t91771: F, t91777: F, t91780: F, t91782: F, t96238: F, t96269: F, t96271: F, t94223: F, t94236: F, t94257: F, t94272: F, t95970: F, t96228: F, t96232: F, t2174: F, t5363: F, t1404: F, t8110: F, t1851: F, t7426: F, t27907: F, t580: F, t2169: F, t5381: F, t16507: F, t16546: F, t2170: F, t3: F, t3932: F, t3946: F, t5364: F, t7416: F, t8111: F, t8119: F, t85405: F, t1395: F, t1858: F, t7415: F, t27930: F, t576: F, t112: F, t2319: F, t1458: F, t16538: F, t2363: F, t24969: F, t24972: F, t27921: F, t4072: F, t577: F, t85423: F, t86582: F, t86606: F, t86610: F, t86612: F, t86614: F, t86616: F, t86619: F, t86622: F, t86625: F, t86629: F, t111: F, t12813: F, t16541: F, t5376: F, t7423: F, t85416: F, t86631: F, t86633: F, t86635: F, t86637: F, t86639: F, t86642: F, t86646: F, t86651: F, t86653: F, t86655: F, t86660: F, t86668: F, t91799: F, t91802: F, t1396: F, t1398: F, t1852: F, t24955: F, t24977: F, t27908: F, t85403: F, t85407: F, t85412: F, t86557: F, t86559: F) -> F {
        let t96274 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2350::<F>(t12823, t15857, t2114, t2312, t2314, t2323, t27290, t27858, t27863, t27879, t4034, t5107, t5361, t574, t652, t671, t672, t7264, t7412, t7989, t8103, t91763, t91765, t91767, t91769, t91771, t91777, t91780, t91782, t96238, t96269, t96271);
        let (t96277, t96281, t96283) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2351::<F>(t94223, t94236, t94257, t94272, t95970, t96228, t96232, t96274, t2174, t5363, t1404, t8110);
        let t96297 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2352::<F>(t1851, t7426, t27907, t580, t2169, t5381, t16507, t16546, t2170, t2174, t3, t3932, t3946, t5364, t7416, t8111, t8119, t85405, t96277, t96281, t96283);
        let (t96300, t96303, t96308, t96327) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2353::<F>(t1395, t8119, t1858, t7415, t27930, t576, t112, t27907, t2169, t2319, t1458, t16538, t2363, t24969, t24972, t27921, t4072, t577, t671, t85423, t86582, t86606, t86610, t86612, t86614, t86616, t86619, t86622, t86625, t86629, t96277);
        let t96337 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2354::<F>(t111, t8110, t12813, t16541, t2319, t24972, t5376, t7423, t85416, t86631, t86633, t86635, t86637, t86639, t86642, t86646, t86651, t86653, t86655, t86660, t86668, t91799, t91802);
        let t96340 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2355::<F>(t1396, t1398, t1404, t1852, t1858, t24955, t24977, t27908, t27930, t85403, t85407, t85412, t86557, t86559, t96300, t96303, t96308, t96327, t96337);
        let tv4rho3sigma5 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2356::<F>(t96297, t96340);
    tv4rho3sigma5
}
