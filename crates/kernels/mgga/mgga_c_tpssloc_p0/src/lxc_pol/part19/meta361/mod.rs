//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1309;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1310;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1311;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1312;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta361<F: Float>(t1041: F, t10918: F, t13969: F, t607: F, t9258: F, t10403: F, t10404: F, t10422: F, t10477: F, t67: F, t3067: F, t11059: F, t10970: F, t820: F, t10418: F, t3070: F, t1021: F, t1023: F, t10305: F, t10316: F, t10321: F, t10408: F, t10426: F, t10483: F, t10883: F, t10886: F, t248: F, t2771: F, t3041: F, t3071: F, t3131: F, t3132: F, t360: F, t42347: F, t42348: F, t42354: F, t42358: F, t42369: F, t4582: F, t4583: F, t884: F, t10397: F, t3120: F, t10517: F, t3103: F, t10868: F, t2780: F, t3051: F, t10277: F, t976: F, t10263: F, t10493: F, t2776: F, t3039: F, t3048: F, t3121: F, t3146: F, t3151: F, t3153: F, t39097: F, t39103: F, t4588: F, t973: F, t974: F, t10993: F, t2960: F, t2244: F, t2250: F) -> (F, F, F, F, F, F, F, F) {
        let (t42372, t42374, t42380, t42386, t42387, t42388) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1309::<F>(t1041, t10918, t13969, t607, t9258, t10403, t10404, t10422, t10477, t67, t3067, t11059);
        let t42409 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1310::<F>(t10970, t820, t10418, t10422, t3070, t1021, t1023, t10305, t10316, t10321, t10403, t10408, t1041, t10426, t10483, t10883, t10886, t248, t2771, t3041, t3071, t3131, t3132, t360, t42347, t42348, t42354, t42358, t42369, t42372, t42374, t42380, t42388, t4582, t4583, t884);
        let (t42412, t42422, t42428, t42432, t42436) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1311::<F>(t10397, t10422, t3070, t3120, t10517, t3103, t1041, t10868, t248, t2780, t10316, t3051);
        let t42459 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1312::<F>(t10277, t976, t1021, t10263, t10403, t1041, t10493, t248, t2776, t3039, t3048, t3070, t3071, t3121, t3132, t3146, t3151, t3153, t360, t39097, t39103, t42374, t42412, t42422, t42428, t42432, t42436, t4582, t4588, t973, t974);
        let (t42460, t42468) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1313::<F>(t10993, t2960, t2244, t2250);
    (t42374, t42386, t42387, t42409, t42422, t42459, t42460, t42468)
}
