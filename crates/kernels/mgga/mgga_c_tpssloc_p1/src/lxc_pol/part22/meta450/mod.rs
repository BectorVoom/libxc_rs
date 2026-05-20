//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1808;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1809;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta450<F: Float>(t120: F, t6347: F, t1352: F, t3805: F, t5187: F, t550: F, t5249: F, t1307: F, t3870: F, t820: F, t19744: F, t19871: F, t5248: F, t12369: F, t12346: F, t12366: F, t12429: F, t1363: F, t16233: F, t16394: F, t16400: F, t19940: F, t19942: F, t19945: F, t19951: F, t19958: F, t19962: F, t19966: F, t19972: F, t19976: F, t19981: F, t3803: F, t5246: F, t5259: F, t6396: F, t19862: F, t19899: F, t19939: F, t553: F, t5287: F, t5335: F, t19739: F, t1332: F, t1336: F, t1381: F, t1383: F, t16060: F, t1814: F, t1838: F, t1840: F, t19756: F, t19761: F, t19763: F, t19805: F, t19810: F, t19813: F, t19815: F, t5230: F, t5234: F, t5339: F, t5341: F, t5344: F, t5345: F, t5351: F, t544: F, t564: F, t6378: F, t6458: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19986, t19989, t19991, t19994) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1808::<F>(t120, t6347, t1352, t3805, t5187, t550, t5249, t1307);
        let (t19996, t20000, t20004, t20007) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1809::<F>(t19994, t3870, t820, t19744, t19871, t5248, t12369, t3805, t12346, t12366, t12429, t1363, t16233, t16394, t16400, t19940, t19942, t19945, t19951, t19958, t19962, t19966, t19972, t19976, t19981, t19986, t19991, t3803, t5246, t5259, t6396);
        let (t20009, t20010, t20014, t20018, t20021) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1810::<F>(t19862, t19899, t19939, t20007, t553, t5287, t5335, t1352, t19739, t1332, t1336, t1381, t1383, t16060, t1814, t1838, t1840, t19756, t19761, t19763, t19805, t19810, t19813, t19815, t5230, t5234, t5339, t5341, t5344, t5345, t5351, t544, t564, t6378, t6458);
    (t19986, t19989, t19991, t19994, t19996, t20000, t20004, t20009, t20010, t20014, t20018, t20021)
}
