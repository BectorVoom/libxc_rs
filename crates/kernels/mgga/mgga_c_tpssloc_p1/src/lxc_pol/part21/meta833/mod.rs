//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta833 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2941;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2942;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2943;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2944;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2945;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2946;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2947;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2948;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2949;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2950;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2951;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta833<F: Float>(t13784: F, t17178: F, t2986: F, t10189: F, t5836: F, t2990: F, t17161: F, t17152: F, t48213: F, t17863: F, t42837: F, t10186: F, t17808: F, t10255: F, t17800: F, t17804: F, t42830: F, t42962: F, t42968: F, t4510: F, t5821: F, t59715: F, t10236: F, t17635: F, t13835: F, t13847: F, t13839: F, t48279: F, t17748: F, t10235: F, t13851: F, t4531: F, t48021: F, t48024: F, t48030: F, t48044: F, t48048: F, t48052: F, t48357: F, t17849: F, t2960: F, t5838: F, t698: F, t973: F, t5844: F, t4540: F, t4509: F, t10190: F, t17794: F, t10237: F, t340: F, t343: F, t4518: F, t48061: F, t48063: F, t48066: F, t48068: F, t48189: F, t59730: F, t974: F, t10263: F, t13769: F, t13831: F, t4347: F, t48207: F, t48210: F, t48215: F, t48233: F, t48242: F, t48244: F, t48250: F, t48256: F, t5839: F, t59767: F, t6733: F, t5842: F, t17686: F, t42841: F, t17783: F, t13779: F, t17167: F, t10241: F, t10245: F, t17817: F, t42846: F, t48281: F, t5818: F, t5825: F, t59659: F, t17171: F, t17157: F, t17769: F, t10224: F, t5824: F, t13822: F, t17752: F, t17753: F, t17758: F, t17778: F, t3008: F, t4546: F, t59755: F, t59763: F, t17757: F, t17772: F, t2970: F, t13931: F, t17773: F, t17841: F, t48292: F, t48297: F, t48302: F, t48317: F, t48320: F, t48328: F, t55677: F, t7577: F, t977: F, t978: F, t984: F, t17777: F, t13798: F, t1597: F, t17857: F, t17860: F, t17864: F, t2978: F, t2994: F, t48336: F, t48338: F, t48342: F, t55723: F, t5829: F, t59751: F, t61065: F, t4514: F, t48019: F, t48046: F, t10259: F, t17742: F, t17745: F, t17749: F, t17801: F, t25608: F, t3014: F, t59719: F, t59746: F, t884: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t61245, t61252, t61258, t61261, t61264, t61273) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2941::<F>(t13784, t17178, t2986, t10189, t5836, t2990, t17161, t17152, t48213, t17863, t42837, t10186, t17808);
        let t61275 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2942::<F>(t10255, t17800, t17804, t2986, t42830, t42962, t42968, t4510, t5821, t59715, t61245, t61252, t61258, t61261, t61264, t61273);
        let t61301 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2943::<F>(t10236, t17635, t13835, t13847, t2986, t13839, t48279, t17748, t10235, t13851, t4531, t48021, t48024, t48030, t48044, t48048, t48052, t48357);
        let (t61307, t61310, t61313, t61315, t61322, t61327) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2944::<F>(t17849, t2960, t5838, t698, t973, t5844, t4540, t4509, t5836, t10190, t17794, t2986);
        let t61332 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2945::<F>(t10237, t2986, t340, t343, t4518, t48061, t48063, t48066, t48068, t48189, t59730, t61307, t61310, t61313, t61315, t61322, t61327, t973, t974);
        let t61355 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2946::<F>(t10263, t13769, t13831, t17800, t2986, t4347, t4518, t4531, t48207, t48210, t48215, t48233, t48242, t48244, t48250, t48256, t5839, t59767, t6733);
        let t61389 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2947::<F>(t4509, t5842, t17686, t42841, t17783, t2960, t13779, t17167, t2986, t10235, t10237, t10241, t10245, t10263, t17804, t17817, t17863, t42846, t4518, t48281, t5818, t5825, t59659);
        let (t61391, t61394, t61397, t61405, t61408) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2948::<F>(t13779, t17171, t2986, t13784, t17157, t10190, t17817, t17769, t2960, t10224, t5824, t973);
        let t61424 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2949::<F>(t13822, t17752, t973, t17753, t17758, t17778, t2960, t2986, t3008, t343, t4510, t4518, t4546, t5842, t59755, t59763, t61391, t61394, t61397, t61405, t61408);
        let t61453 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2950::<F>(t13822, t17757, t973, t17772, t2970, t13931, t17773, t17841, t2960, t343, t4546, t48292, t48297, t48302, t48317, t48320, t48328, t55677, t7577, t977, t978, t984);
        let t61485 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2951::<F>(t13822, t17777, t973, t10186, t10263, t13798, t13839, t1597, t17857, t17860, t17864, t2978, t2986, t2994, t3008, t343, t4546, t48336, t48338, t48342, t55723, t5829, t5836, t59751, t61065, t977, t984);
        let t61523 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2952::<F>(t2986, t4514, t48019, t48046, t10186, t10259, t17742, t17745, t17749, t17794, t17801, t17817, t25608, t3014, t343, t4510, t4518, t4531, t4546, t5836, t59719, t59746, t884, t973);
    (t61275, t61301, t61332, t61355, t61389, t61424, t61453, t61485, t61523)
}
