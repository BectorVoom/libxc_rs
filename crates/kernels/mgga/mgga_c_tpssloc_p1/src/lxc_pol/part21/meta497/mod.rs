//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2112;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2113;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2114;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta497<F: Float>(t1527: F, t4300: F, t2718: F, t17050: F, t17052: F, t17057: F, t17060: F, t17064: F, t259: F, t2597: F, t2713: F, t4147: F, t4268: F, t4273: F, t4301: F, t5637: F, t5658: F, t855: F, t866: F, t16804: F, t252: F, t1492: F, t4265: F, t225: F, t5632: F, t5561: F, t1519: F, t4142: F, t5631: F, t798: F, t5558: F, t852: F, t13042: F, t13053: F, t13065: F, t13463: F, t1528: F, t2752: F, t5660: F, t13105: F, t16685: F, t16688: F, t16691: F, t16692: F, t16695: F, t16696: F, t1877: F, t193: F, t202: F, t4303: F, t4307: F, t868: F, t870: F, t9789: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17070, t17079) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2112::<F>(t1527, t4300, t2718, t17050, t17052, t17057, t17060, t17064, t259, t2597, t2713, t4147, t4268, t4273, t4301, t5637, t5658, t855, t866);
        let (t17083, t17087, t17090, t17092, t17095, t17098, t17100) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2113::<F>(t16804, t252, t1492, t4265, t225, t5632, t5561, t1519, t4142, t5631, t798, t5558, t852);
        let t17108 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2114::<F>(t13042, t13053, t13065, t13463, t1528, t17083, t17087, t17090, t17092, t17095, t17098, t17100, t259, t2597, t4268, t4273, t5658, t866);
        let (t17109, t17116, t17119) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2115::<F>(t17079, t17108, t2752, t5660, t13105, t16685, t16688, t16691, t16692, t16695, t16696, t1877, t193, t202, t4303, t4307, t868, t870, t9789, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
    (t17070, t17083, t17087, t17090, t17092, t17095, t17098, t17100, t17109, t17116, t17119)
}
