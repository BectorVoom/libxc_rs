//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1879;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1880;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta479<F: Float>(t1495: F, t210: F, t5544: F, t10026: F, t10029: F, t13368: F, t16942: F, t16954: F, t16988: F, t16990: F, t16993: F, t16995: F, t17000: F, t2571: F, t13087: F, t13182: F, t13234: F, t16848: F, t16877: F, t16879: F, t20882: F, t20887: F, t20891: F, t20896: F, t20958: F, t20998: F, t2643: F, t843: F, t235: F, t20986: F, t4282: F, t4295: F, t5612: F, t1499: F, t1523: F, t1525: F, t16673: F, t20806: F, t20854: F, t20858: F, t20862: F, t20867: F, t20871: F, t20873: F, t20876: F, t20937: F, t226: F, t255: F, t4166: F, t4281: F, t4291: F, t5575: F, t5645: F, t5648: F, t5651: F, t5653: F, t5655: F, t812: F) -> (F, F, F, F, F, F) {
        let (t21008, t21011) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1879::<F>(t1495, t210, t5544, t10026, t10029, t13368, t16942, t16954, t16988, t16990, t16993, t16995, t17000, t2571);
        let t21013 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1880::<F>(t13087, t13182, t13234, t16848, t16877, t16879, t20882, t20887, t20891, t20896, t20958, t20998, t21011, t2643, t843);
        let (t21014, t21025, t21028, t21033) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1881::<F>(t21013, t235, t20986, t4282, t4295, t5612, t1499, t1523, t1525, t16673, t20806, t20854, t20858, t20862, t20867, t20871, t20873, t20876, t20937, t226, t255, t4166, t4281, t4291, t5575, t5645, t5648, t5651, t5653, t5655, t812);
    (t21008, t21013, t21014, t21025, t21028, t21033)
}
