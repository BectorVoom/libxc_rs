//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta421 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1609;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1610;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1611;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1612;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1613;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1614;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1615;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1616;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1617;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1618;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1619;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1620;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta421<F: Float>(t15453: F, t17686: F, t4582: F, t17635: F, t4972: F, t1090: F, t6230: F, t3578: F, t6219: F, t4997: F, t5002: F, t11784: F, t248: F, t5971: F, t1227: F, t5019: F, t4993: F, t5005: F, t1202: F, t6164: F, t5024: F, t11692: F, t11792: F, t11821: F, t15671: F, t15691: F, t15699: F, t15740: F, t3577: F, t488: F, t4950: F, t1196: F, t16558: F, t974: F, t1215: F, t1653: F, t15659: F, t1177: F, t18221: F, t18237: F, t1735: F, t4724: F, t11668: F, t18232: F, t3440: F, t1017: F, t6163: F, t1210: F, t1207: F, t11665: F, t11678: F, t1174: F, t11834: F, t1218: F, t15569: F, t15717: F, t15719: F, t15722: F, t4889: F, t4954: F, t4969: F, t5046: F, t6192: F, t372: F, t479: F, t471: F, t3521: F, t5979: F, t1009: F, t6150: F, t1011: F, t1212: F, t1226: F, t6169: F, t486: F, t6218: F, t4978: F, t1216: F, t4987: F, t4977: F, t5012: F, t11836: F, t1232: F, t15495: F, t15727: F, t15731: F, t15735: F, t15745: F, t1737: F, t3506: F, t3515: F, t3536: F, t4989: F, t6221: F, t18300: F, t5001: F, t5018: F, t1730: F, t5023: F, t18225: F, t1193: F, t6109: F, t3570: F, t1230: F, t18241: F, t11546: F, t18206: F, t11738: F, t15591: F, t15594: F, t15754: F, t1748: F, t3490: F, t5014: F, t5030: F, t5033: F, t6207: F, t6211: F, t18316: F, t18337: F, t18390: F, t18951: F, t466: F, t5068: F, t6260: F, t18940: F, t491: F, t1246: F, t5079: F, t6256: F, t3625: F, t5011: F, t1755: F, t1235: F, t6224: F, t475: F, t6739: F, t6252: F, t11889: F, t11888: F, t11904: F, t11907: F, t11914: F, t1244: F, t15027: F, t15032: F, t15245: F, t1756: F, t3604: F, t3610: F, t3624: F, t5064: F, t5069: F, t5080: F, t5084: F, t6253: F, t6261: F, t6263: F, t11883: F, t1751: F, t6238: F, t3612: F, t1734: F, t5052: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18955, t18959, t18965, t18969, t18972, t18975) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1609::<F>(t15453, t17686, t4582, t17635, t4972, t1090, t6230, t3578, t6219, t4997, t5002, t11784, t248, t5971);
        let t18989 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1610::<F>(t1227, t18975, t4997, t5019, t4993, t5005, t1202, t6164, t5024, t11692, t11792, t11821, t15671, t15691, t15699, t15740, t18955, t18959, t18965, t18969, t18972, t3577, t488, t4950);
        let (t18997, t19002, t19005, t19010, t19015) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1611::<F>(t1196, t16558, t974, t1215, t1653, t15659, t3578, t1177, t18221, t18237, t1735, t4724);
        let t19029 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1612::<F>(t11668, t19015, t18232, t3440, t1017, t6163, t1210, t1207, t11665, t11678, t1174, t11834, t1218, t15569, t15717, t15719, t15722, t15740, t18997, t19002, t19005, t19010, t3577, t4889, t4950, t4954, t4969, t5046, t6192);
        let (t19033, t19041, t19045, t19047) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1613::<F>(t372, t6163, t479, t471, t248, t3521, t5979, t1227, t1009, t6150, t1011, t1212);
        let t19075 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1614::<F>(t1226, t6169, t486, t6218, t4978, t4582, t1216, t17635, t4987, t4977, t5012, t11836, t1218, t1227, t1232, t15495, t15727, t15731, t15735, t15745, t1737, t19033, t19041, t19047, t3506, t3515, t3536, t4989, t5024, t6221);
        let (t19077, t19080, t19083, t19087, t19090, t19095) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1615::<F>(t1216, t18300, t4582, t5001, t5018, t1730, t5023, t1177, t18225, t1193, t6109, t248, t3570, t6230);
        let t19117 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1616::<F>(t19095, t3515, t1230, t18241, t248, t11546, t18206, t11738, t1174, t1218, t1227, t1232, t15591, t15594, t15754, t1737, t1748, t19077, t19080, t19083, t19087, t19090, t3490, t4889, t5002, t5005, t5014, t5030, t5033, t6207, t6211);
        let (t19120, t19121, t19123, t19128) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1617::<F>(t18316, t18337, t18390, t18951, t18989, t19029, t19075, t19117, t466, t5068, t6260, t18940, t491);
        let (t19129, t19131, t19139, t19142, t19145, t19146, t19153) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1618::<F>(t1246, t19128, t5079, t6256, t3625, t5011, t1755, t5068, t1235, t6224, t1215, t475, t6739);
        let t19164 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1619::<F>(t19153, t6252, t11889, t1215, t5079, t6260, t11888, t11904, t11907, t11914, t1244, t15027, t15032, t15245, t1756, t19123, t19129, t19131, t19139, t19142, t19146, t3604, t3610, t3624, t5064, t5069, t5080, t5084, t6253, t6261, t6263);
        let (t19166, t19170, t19174, t19176, t19180, t19189) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1620::<F>(t11883, t1215, t6252, t1751, t5011, t1246, t6238, t19145, t3612, t1734, t5052, t1235, t6218);
    (t19045, t19120, t19121, t19164, t19166, t19170, t19174, t19176, t19180, t19189)
}
