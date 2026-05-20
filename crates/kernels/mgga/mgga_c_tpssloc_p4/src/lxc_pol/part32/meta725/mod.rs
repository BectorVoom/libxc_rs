//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta725 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2329;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2330;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2331;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2332;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2333;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2334;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2335;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2336;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2337;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2338;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2339;
use chunk11::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta725<F: Float>(t1210: F, t24721: F, t29593: F, t27700: F, t95422: F, t2132: F, t2136: F, t5398: F, t19040: F, t7345: F, t18392: F, t27617: F, t4993: F, t18215: F, t2121: F, t2133: F, t24736: F, t27703: F, t4899: F, t6138: F, t6203: F, t7321: F, t8027: F, t95540: F, t95542: F, t95545: F, t28525: F, t461: F, t7324: F, t18342: F, t18346: F, t18590: F, t18594: F, t18997: F, t19068: F, t27604: F, t27674: F, t4974: F, t4989: F, t5046: F, t7310: F, t7331: F, t95550: F, t95571: F, t95573: F, t210: F, t29584: F, t27683: F, t27710: F, t1198: F, t27684: F, t27692: F, t27711: F, t6192: F, t8040: F, t86330: F, t86348: F, t86350: F, t95323: F, t95556: F, t95587: F, t95590: F, t95593: F, t95617: F, t95588: F, t18975: F, t18332: F, t1222: F, t29606: F, t1748: F, t18584: F, t24741: F, t27580: F, t27655: F, t27687: F, t27714: F, t5030: F, t6232: F, t7999: F, t8031: F, t8035: F, t86167: F, t95452: F, t95662: F, t95702: F, t104029: F, t104056: F, t104087: F, t104101: F, t104134: F, t104155: F, t104193: F, t104220: F, t104264: F, t104292: F, t104319: F, t104351: F, t29787: F, t85639: F, t1170: F, t29726: F, t103337: F, t1244: F, t1246: F, t15027: F, t1716: F, t19201: F, t2147: F, t27454: F, t27471: F, t27507: F, t27511: F, t27543: F, t27725: F, t470: F, t491: F, t4928: F, t493: F, t5064: F, t6218: F, t7283: F, t7348: F, t7387: F, t95768: F, t95774: F, t24574: F, t29557: F, t29551: F, t8003: F, t94490: F, t103218: F, t103490: F, t103494: F, t103538: F, t103577: F, t103624: F, t103659: F, t103693: F, t103733: F, t103766: F, t103801: F, t103829: F, t103864: F, t103889: F, t103918: F, t103949: F, t103978: F, t104002: F, t1186: F, t1238: F, t1241: F, t14980: F, t2122: F, t24567: F, t24638: F, t27411: F, t27751: F, t29545: F, t29670: F, t497: F, t6146: F, t7303: F, t8088: F, t94710: F, t29694: F, t29678: F, t7280: F, t14972: F, t15820: F, t1761: F, t18571: F, t2144: F, t24893: F, t27383: F, t27396: F, t27406: F, t27427: F, t29795: F, t3487: F, t4945: F, t498: F, t6150: F, t6268: F, t86451: F, t94759: F, t95899: F, t225: F, t29687: F, t1252: F, t1721: F, t2155: F, t254: F, t27549: F, t27742: F, t27761: F, t27767: F, t27775: F, t27779: F, t27786: F, t29532: F, t3593: F, t466: F, t5055: F, t65208: F, t94514: F, t94779: F, t95824: F, t95902: F, t27817: F, t15797: F, t1751: F, t17635: F, t19209: F, t19220: F, t19232: F, t24589: F, t24601: F, t27444: F, t27453: F, t27766: F, t3598: F, t460: F, t4940: F, t6267: F, t7286: F, t7351: F, t7391: F, t7392: F, t8054: F, t8061: F, t86473: F, t95834: F, t29813: F, t1760: F, t19213: F, t19219: F, t19225: F, t24602: F, t24615: F, t27389: F, t27741: F, t27784: F, t27785: F, t27830: F, t3966: F, t5060: F, t7300: F, t8002: F, t94358: F, t94680: F, t95863: F, t95866: F, t95884: F, t95889: F) -> (F, F, F, F, F) {
        let (t104355, t104364, t104367, t104369, t104371, t104375) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2329::<F>(t1210, t24721, t29593, t27700, t95422, t2132, t2136, t5398, t19040, t7345, t18392, t27617, t4993);
        let t104380 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2330::<F>(t104355, t104364, t104367, t104369, t104371, t104375, t18215, t2121, t2132, t2133, t24736, t27703, t4899, t6138, t6203, t7321, t8027, t95540, t95542, t95545);
        let t104404 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2331::<F>(t28525, t461, t7324, t18342, t18346, t18590, t18594, t18997, t19068, t27604, t27617, t27674, t4974, t4989, t5046, t7310, t7331, t7345, t95550, t95571, t95573);
        let t104424 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2332::<F>(t210, t29584, t27683, t27710, t1198, t27684, t27692, t27711, t6192, t7331, t8040, t86330, t86348, t86350, t95323, t95556, t95587, t95590, t95593, t95617);
        let t104449 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2333::<F>(t27700, t95588, t18975, t7345, t18332, t7310, t1222, t29606, t1748, t18584, t24741, t27580, t27604, t27655, t27687, t27714, t5030, t6232, t7999, t8031, t8035, t86167, t95452, t95662, t95702);
        let t104453 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2334::<F>(t104029, t104056, t104087, t104101, t104134, t104155, t104193, t104220, t104264, t104292, t104319, t104351, t104380, t104404, t104424, t104449);
        let t104482 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2335::<F>(t29787, t85639, t1170, t2121, t29726, t103337, t104453, t1244, t1246, t15027, t1716, t19201, t2147, t27454, t27471, t27507, t27511, t27543, t27725, t470, t491, t4928, t493, t5064, t6218, t7283, t7348, t7387, t95768, t95774);
        let t104508 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2336::<F>(t24574, t29557, t29551, t8003, t94490, t103218, t103490, t103494, t103538, t103577, t103624, t103659, t103693, t103733, t103766, t103801, t103829, t103864, t103889, t103918, t103949, t103978, t104002, t104482, t1186, t1238, t1241, t14980, t1716, t2122, t24567, t24638, t27411, t27751, t29545, t29670, t4928, t497, t6146, t7283, t7303, t8088, t94710);
        let t104534 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2337::<F>(t24574, t29694, t1170, t2121, t29670, t29678, t7280, t14972, t15820, t1761, t18571, t2144, t24893, t27383, t27396, t27406, t27427, t29795, t3487, t4945, t498, t6150, t6268, t7348, t8088, t86451, t94759, t95899);
        let t104564 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2338::<F>(t225, t29687, t104453, t1252, t1721, t1761, t2155, t254, t27396, t27406, t27549, t27742, t27761, t27767, t27775, t27779, t27786, t29532, t3593, t466, t498, t5055, t65208, t7999, t94514, t94779, t95824, t95902);
        let t104596 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2339::<F>(t27817, t7999, t1238, t14972, t15797, t1716, t1751, t17635, t19209, t19220, t19232, t24589, t24601, t27444, t27453, t27766, t29795, t3593, t3598, t460, t4940, t498, t6267, t7283, t7286, t7351, t7391, t7392, t8054, t8061, t86473, t95834);
        let t104631 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2340::<F>(t24574, t29813, t1238, t14980, t1760, t19213, t19219, t19225, t24589, t24601, t24602, t24615, t27389, t27406, t27741, t27784, t27785, t27830, t3598, t3966, t5060, t7283, t7300, t8002, t8061, t94358, t94680, t95863, t95866, t95884, t95889);
    (t104508, t104534, t104564, t104596, t104631)
}
