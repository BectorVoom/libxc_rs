//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta668 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1965;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1966;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1967;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1968;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1969;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1970;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1971;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1972;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1973;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1974;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1975;
use chunk11::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta668<F: Float>(t225: F, t29095: F, t26729: F, t866: F, t86930: F, t86931: F, t92415: F, t92425: F, t98202: F, t98205: F, t98213: F, t98222: F, t98227: F, t98279: F, t92578: F, t98610: F, t98612: F, t98614: F, t98616: F, t98618: F, t98620: F, t98622: F, t98624: F, t98626: F, t98629: F, t98631: F, t98633: F, t98635: F, t98637: F, t98639: F, t98642: F, t84857: F, t84859: F, t87213: F, t92580: F, t92582: F, t98647: F, t98651: F, t98655: F, t98659: F, t98663: F, t98668: F, t98672: F, t98674: F, t98676: F, t98678: F, t98680: F, t98682: F, t98685: F, t81789: F, t87237: F, t87243: F, t87268: F, t92590: F, t92599: F, t92603: F, t92607: F, t92614: F, t92615: F, t98690: F, t98694: F, t98696: F, t98701: F, t98703: F, t98707: F, t98709: F, t98711: F, t84896: F, t84897: F, t87304: F, t87306: F, t92626: F, t92627: F, t92630: F, t98715: F, t98717: F, t98719: F, t98721: F, t98723: F, t98725: F, t98728: F, t98731: F, t98733: F, t98736: F, t98738: F, t87319: F, t87320: F, t92635: F, t92645: F, t98744: F, t98746: F, t98750: F, t98752: F, t98754: F, t98758: F, t98762: F, t98766: F, t98770: F, t98774: F, t98777: F, t98782: F, t98787: F, t98791: F, t81903: F, t87335: F, t87345: F, t87387: F, t92646: F, t92647: F, t92649: F, t92650: F, t92653: F, t92657: F, t92675: F, t98796: F, t98798: F, t98801: F, t98803: F, t98808: F, t98811: F, t98814: F, t87403: F, t87405: F, t87414: F, t87425: F, t87432: F, t92679: F, t98818: F, t98820: F, t98822: F, t98824: F, t98826: F, t98828: F, t98830: F, t98833: F, t98836: F, t98838: F, t98842: F, t98844: F, t84921: F, t84932: F, t87437: F, t87438: F, t87440: F, t87445: F, t92697: F, t92705: F, t92710: F, t92713: F, t98847: F, t98849: F, t98851: F, t98853: F, t98858: F, t98862: F, t98868: F, t98871: F, t1528: F, t17056: F, t218: F, t25168: F, t259: F, t26728: F, t2713: F, t29091: F, t86983: F, t86991: F, t86994: F, t92386: F, t98251: F, t98256: F, t98264: F, t98277: F, t29099: F, t13463: F, t17057: F, t17063: F, t17092: F, t26582: F, t4268: F, t7087: F, t7107: F, t7830: F, t87042: F, t87050: F, t92394: F, t92486: F, t98315: F, t98319: F, t98322: F, t10109: F, t7841: F, t13065: F, t17052: F, t17090: F, t2054: F, t24305: F, t26703: F, t26713: F, t4147: F, t4272: F, t4301: F, t5658: F, t59498: F, t7092: F, t7842: F, t85101: F, t87779: F, t92846: F, t92847: F, t92862: F, t92866: F, t92872: F, t98921: F, t98923: F, t98927: F) -> (F, F, F, F, F, F) {
        let t101359 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1965::<F>(t225, t29095, t26729, t866, t86930, t86931, t92415, t92425, t98202, t98205, t98213, t98222, t98227, t98279);
        let t101398 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1966::<F>(t92578, t98610, t98612, t98614, t98616, t98618, t98620, t98622, t98624, t98626, t98629, t98631, t98633, t98635, t98637, t98639, t98642);
        let t101413 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1967::<F>(t84857, t84859, t87213, t92580, t92582, t98647, t98651, t98655, t98659, t98663, t98668, t98672, t98674, t98676, t98678, t98680, t98682, t98685);
        let t101425 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1968::<F>(t81789, t87237, t87243, t87268, t92590, t92599, t92603, t92607, t92614, t92615, t98690, t98694, t98696, t98701, t98703, t98707, t98709, t98711);
        let t101439 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1969::<F>(t84896, t84897, t87304, t87306, t92626, t92627, t92630, t98715, t98717, t98719, t98721, t98723, t98725, t98728, t98731, t98733, t98736, t98738);
        let t101456 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1970::<F>(t87319, t87320, t92635, t92645, t98744, t98746, t98750, t98752, t98754, t98758, t98762, t98766, t98770, t98774, t98777, t98782, t98787, t98791);
        let t101468 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1971::<F>(t81903, t87335, t87345, t87387, t92646, t92647, t92649, t92650, t92653, t92657, t92675, t98796, t98798, t98801, t98803, t98808, t98811, t98814);
        let t101486 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1972::<F>(t87403, t87405, t87414, t87425, t87432, t92679, t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98833, t98836, t98838, t98842, t98844);
        let t101496 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1973::<F>(t84921, t84932, t87437, t87438, t87440, t87445, t92697, t92705, t92710, t92713, t98847, t98849, t98851, t98853, t98858, t98862, t98868, t98871);
        let (t101499, t101504) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1974::<F>(t101398, t101413, t101425, t101439, t101456, t101468, t101486, t101496, t1528, t17056, t218, t25168, t259, t26728, t2713, t29091, t86983, t86991, t86994, t92386, t98251, t98256, t98264, t98277);
        let (t101509, t101540) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1975::<F>(t225, t29099, t13463, t17057, t17063, t17092, t25168, t26582, t4268, t7087, t7107, t7830, t87042, t87050, t92394, t92486, t98315, t98319, t98322);
        let t101569 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1976::<F>(t10109, t7841, t13065, t13463, t1528, t17052, t17090, t2054, t24305, t25168, t26703, t26713, t4147, t4272, t4301, t5658, t59498, t7092, t7107, t7830, t7842, t85101, t87779, t92846, t92847, t92862, t92866, t92872, t98921, t98923, t98927);
    (t101359, t101499, t101504, t101509, t101540, t101569)
}
