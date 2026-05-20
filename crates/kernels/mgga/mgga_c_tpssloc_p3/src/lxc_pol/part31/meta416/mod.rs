//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1521;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1522;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta416<F: Float>(t19862: F, t19899: F, t19939: F, t20007: F, t553: F, t5287: F, t5335: F, t1352: F, t19739: F, t1332: F, t1336: F, t1381: F, t1383: F, t16060: F, t1814: F, t1838: F, t1840: F, t19756: F, t19761: F, t19763: F, t19805: F, t19810: F, t19813: F, t19815: F, t5230: F, t5234: F, t5339: F, t5341: F, t5344: F, t5345: F, t5351: F, t544: F, t564: F, t6378: F, t6458: F, t19755: F, t1378: F, t1385: F, t6460: F, t3887: F, t225: F, t6364: F, t539: F, t1375: F, t1386: F, t16030: F, t16439: F, t1843: F, t19635: F, t19644: F, t19648: F, t3882: F, t5321: F, t5326: F, t5354: F, t568: F, t6461: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20009, t20014, t20018, t20021) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1521::<F>(t19862, t19899, t19939, t20007, t553, t5287, t5335, t1352, t19739, t1332, t1336, t1381, t1383, t16060, t1814, t1838, t1840, t19756, t19761, t19763, t19805, t19810, t19813, t19815, t5230, t5234, t5339, t5341, t5344, t5345, t5351, t544, t564, t6378, t6458);
        let (t20022, t20023, t20025, t20026, t20029, t20032, t20034) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1522::<F>(t19755, t20021, t1378, t1385, t6460, t3887, t225, t6364, t20009, t539, t1375, t1386, t16030, t16439, t1843, t19635, t19644, t19648, t3882, t5321, t5326, t5354, t568, t6461);
    (t20009, t20014, t20018, t20022, t20023, t20025, t20026, t20029, t20032, t20034)
}
