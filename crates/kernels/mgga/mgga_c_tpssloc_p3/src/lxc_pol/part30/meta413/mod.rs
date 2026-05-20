//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1569;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1570;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1571;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta413<F: Float>(t4781: F, t4785: F, t3313: F, t11277: F, t5988: F, t1117: F, t11275: F, t3411: F, t6106: F, t1157: F, t6105: F, t1164: F, t11282: F, t6068: F, t11285: F, t1155: F, t11292: F, t4883: F, t15218: F, t4882: F, t1190: F, t6238: F, t1743: F, t4965: F, t486: F, t6224: F, t11721: F, t1215: F, t4582: F, t4978: F, t1222: F, t6170: F, t6158: F, t6165: F, t11644: F, t11649: F, t11719: F, t11728: F, t15446: F, t15448: F, t15450: F, t15452: F, t15503: F, t15507: F, t488: F, t4974: F, t4980: F, t4984: F, t5005: F, t5416: F, t972: F, t135: F, t6187: F, t1174: F, t4889: F, t5040: F, t6183: F, t6177: F, t1198: F, t15484: F, t15488: F, t15490: F, t15494: F, t15498: F, t15524: F, t15550: F, t15574: F, t15580: F, t15737: F, t1748: F, t5024: F, t5030: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18264, t18268, t18270, t18273) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1569::<F>(t4781, t4785, t3313, t11277, t5988, t1117, t11275, t3411, t6106, t1157, t6105, t1164);
        let (t18278, t18282, t18285, t18287, t18297) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1570::<F>(t11282, t6068, t11285, t1155, t1164, t11292, t4883, t15218, t4882, t1190, t6238, t1743, t4965);
        let (t18300, t18316) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1571::<F>(t486, t6224, t11721, t1215, t4582, t4978, t1222, t6170, t6158, t6165, t11644, t11649, t11719, t11728, t15446, t15448, t15450, t15452, t15503, t15507, t18297, t488, t4974, t4980, t4984, t5005);
        let (t18321, t18337) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1572::<F>(t5416, t972, t135, t6187, t1174, t4889, t5040, t6183, t6177, t1198, t15484, t15488, t15490, t15494, t15498, t15524, t15550, t15574, t15580, t15737, t1748, t4980, t5024, t5030);
    (t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18287, t18300, t18316, t18321, t18337)
}
