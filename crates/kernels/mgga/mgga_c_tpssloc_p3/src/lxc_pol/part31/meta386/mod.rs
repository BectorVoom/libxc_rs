//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta386 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1365;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1366;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1367;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1368;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1369;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1370;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1371;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1372;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1373;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1374;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta386<F: Float>(t135: F, t5889: F, t973: F, t5893: F, t5884: F, t4593: F, t4650: F, t4582: F, t5398: F, t607: F, t4583: F, t1041: F, t13948: F, t13952: F, t13959: F, t13963: F, t13966: F, t13972: F, t2960: F, t3039: F, t5885: F, t5890: F, t5894: F, t4588: F, t1023: F, t5681: F, t3071: F, t248: F, t3101: F, t5878: F, t3051: F, t5685: F, t4630: F, t4641: F, t5873: F, t3130: F, t376: F, t5872: F, t1022: F, t10482: F, t1539: F, t5867: F, t884: F, t10390: F, t10480: F, t10904: F, t13995: F, t14000: F, t14027: F, t3070: F, t4575: F, t5875: F, t5909: F, t5392: F, t14172: F, t1409: F, t3966: F, t14187: F, t1616: F, t4347: F, t5866: F, t4594: F, t10413: F, t10436: F, t10511: F, t10871: F, t14049: F, t14059: F, t3114: F, t4585: F, t4590: F, t4644: F, t5869: F, t3131: F, t4649: F, t16558: F, t998: F, t974: F, t13835: F, t4531: F, t13769: F, t13839: F, t6733: F, t4540: F, t7577: F, t4546: F, t343: F, t5842: F, t984: F, t2970: F, t5824: F, t10226: F, t13782: F, t13787: F, t13790: F, t13825: F, t2986: F, t5825: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t17616, t17621, t17625, t17632, t17635) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1365::<F>(t135, t5889, t973, t5893, t5884, t4593, t4650, t4582, t5398, t607);
        let t17640 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1366::<F>(t17635, t4583, t4582, t1041, t13948, t13952, t13959, t13963, t13966, t13972, t17616, t17621, t17625, t17632, t2960, t3039, t5885, t5890, t5894);
        let (t17643, t17649, t17656, t17659) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1367::<F>(t17635, t4588, t4582, t1023, t5681, t3071, t248, t3101, t5878, t3039, t3051, t5685);
        let (t17660, t17662, t17668, t17670, t17671) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1368::<F>(t1041, t17659, t4630, t4641, t248, t3101, t5873, t3130, t376, t5872, t1022, t10482);
        let t17684 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1369::<F>(t17670, t17671, t4582, t1539, t4650, t3071, t5867, t884, t10390, t1041, t10480, t10904, t13995, t14000, t14027, t17643, t17649, t17656, t17660, t17662, t17668, t3070, t4575, t5875, t5909);
        let t17686 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1370::<F>(t5392, t607);
        let (t17688, t17691, t17693, t17697, t17701, t17704) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1371::<F>(t14172, t17686, t4582, t1409, t3966, t4588, t14187, t5878, t884, t3071, t1616, t4347);
        let t17725 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1372::<F>(t17704, t3071, t376, t5866, t4594, t4582, t1023, t1041, t10413, t10436, t10511, t10871, t14049, t14059, t17688, t17693, t17697, t17701, t3039, t3070, t3114, t3130, t4585, t4590, t4644, t5869);
        let (t17734, t17738, t17742, t17745, t17748) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1373::<F>(t3131, t4649, t4593, t4582, t16558, t998, t974, t13835, t4531, t13769, t13839, t1539, t6733);
        let t17766 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1374::<F>(t17748, t4531, t4540, t7577, t4546, t343, t5842, t984, t2970, t5824, t973, t10226, t13782, t13787, t13790, t13825, t17742, t17745, t2960, t2986, t5825);
    (t17635, t17640, t17670, t17684, t17686, t17691, t17725, t17734, t17738, t17766)
}
