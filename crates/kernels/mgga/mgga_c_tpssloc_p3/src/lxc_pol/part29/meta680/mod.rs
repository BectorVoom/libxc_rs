//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta680 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2286;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2287;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2288;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2289;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2290;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2291;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2292;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta680<F: Float>(t24574: F, t27383: F, t7288: F, t94490: F, t11613: F, t1190: F, t15820: F, t24634: F, t24880: F, t24883: F, t24887: F, t27406: F, t27426: F, t27721: F, t27742: F, t27747: F, t3481: F, t3487: F, t3593: F, t498: F, t5089: F, t7283: F, t7356: F, t8054: F, t8061: F, t86390: F, t27438: F, t85639: F, t225: F, t27419: F, t1236: F, t1252: F, t12652: F, t1409: F, t15797: F, t2128: F, t24589: F, t24590: F, t24601: F, t24602: F, t24626: F, t24638: F, t24877: F, t254: F, t27388: F, t27444: F, t27786: F, t3630: F, t4936: F, t4945: F, t7392: F, t27427: F, t5052: F, t7284: F, t14980: F, t15803: F, t1761: F, t2155: F, t24868: F, t27382: F, t3477: F, t5055: F, t51928: F, t7287: F, t7351: F, t86400: F, t86409: F, t86424: F, t27779: F, t8015: F, t85660: F, t27826: F, t11606: F, t11925: F, t1238: F, t15771: F, t15789: F, t2121: F, t24564: F, t24591: F, t27549: F, t27774: F, t27784: F, t27785: F, t27792: F, t3598: F, t3599: F, t3600: F, t462: F, t497: F, t5088: F, t53658: F, t7391: F, t8087: F, t8088: F, t86426: F, t94395: F, t1751: F, t24594: F, t27403: F, t1251: F, t14706: F, t15425: F, t15786: F, t1716: F, t2144: F, t2154: F, t24596: F, t24893: F, t27741: F, t4930: F, t5060: F, t51925: F, t7285: F, t7286: F, t85688: F, t86451: F, t86456: F, t27389: F, t8074: F, t85917: F, t24826: F, t27511: F, t15394: F, t2127: F, t221: F, t11147: F, t491: F, t1235: F, t12648: F, t14165: F, t14988: F, t15240: F, t24788: F, t24789: F, t27461: F, t27473: F, t27550: F, t27561: F, t3247: F, t3961: F, t7373: F, t7375: F, t7376: F, t1089: F, t7327: F, t1653: F, t7330: F, t85822: F, t131: F, t1419: F, t23598: F, t467: F, t15702: F, t15776: F, t1755: F, t24667: F, t24785: F, t24817: F, t24823: F, t24849: F, t24852: F, t27507: F, t27531: F, t27551: F, t27643: F, t3248: F, t3252: F, t8066: F, t85820: F, t86015: F, t86037: F, t86059: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t94637 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2286::<F>(t24574, t27383, t7288, t94490, t11613, t1190, t15820, t24634, t24880, t24883, t24887, t27406, t27426, t27721, t27742, t27747, t3481, t3487, t3593, t498, t5089, t7283, t7356, t8054, t8061, t86390);
        let t94673 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2287::<F>(t27438, t85639, t225, t27419, t1236, t1252, t12652, t1409, t15797, t15820, t2128, t24589, t24590, t24601, t24602, t24626, t24638, t24877, t254, t27388, t27406, t27444, t27747, t27786, t3487, t3630, t4936, t4945, t7356, t7392);
        let t94698 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2288::<F>(t24574, t27427, t5052, t7284, t14980, t15803, t1761, t2155, t24868, t27382, t27742, t3477, t3593, t4945, t5055, t51928, t7283, t7287, t7351, t7356, t7392, t86400, t86409, t86424);
        let t94734 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2289::<F>(t24574, t27779, t8015, t85660, t27826, t11606, t11925, t1238, t12652, t15771, t15789, t2121, t2155, t225, t24564, t24591, t24601, t27406, t27549, t27774, t27784, t27785, t27792, t3598, t3599, t3600, t462, t497, t5088, t53658, t7391, t8087, t8088, t86426, t94395);
        let t94770 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2290::<F>(t1751, t24594, t24574, t27403, t1238, t1251, t14706, t15425, t15786, t1716, t2144, t2154, t2155, t24596, t24638, t24880, t24893, t27741, t3598, t4930, t498, t5060, t5089, t51925, t7283, t7285, t7286, t85688, t86451, t86456);
        let (t94779, t94796, t94820) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2291::<F>(t24574, t27389, t8074, t85917, t24826, t27511, t15394, t2127, t221, t11147, t491, t1235, t12648, t12652, t14165, t14988, t15240, t24589, t24788, t24789, t27461, t27473, t27550, t27561, t3247, t3961, t7373, t7375, t7376, t94395);
        let (t94850, t94867) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2292::<F>(t1089, t1751, t7327, t1653, t7330, t85822, t3961, t131, t1419, t23598, t467, t14165, t15702, t15776, t1755, t24589, t24667, t24785, t24817, t24823, t24849, t24852, t27507, t27531, t27550, t27551, t27643, t3248, t3252, t7373, t7375, t7376, t8066, t85820, t86015, t86037, t86059);
    (t94637, t94673, t94698, t94734, t94770, t94779, t94796, t94820, t94850, t94867)
}
