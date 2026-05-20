//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta380 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1556;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1557;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1558;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1559;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1560;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1561;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1562;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1563;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta380<F: Float>(t14228: F, t4337: F, t10408: F, t13510: F, t13512: F, t13514: F, t13517: F, t13519: F, t13522: F, t13524: F, t13526: F, t13657: F, t13661: F, t13665: F, t13720: F, t13722: F, t13726: F, t13729: F, t13731: F, t13734: F, t13566: F, t13602: F, t10556: F, t10558: F, t10560: F, t10562: F, t10636: F, t13563: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F, t291: F, t10629: F, t1580: F, t10632: F, t2906: F, t959: F, t1573: F, t2904: F, t4408: F, t923: F, t1561: F, t2885: F, t2860: F, t10760: F, t1569: F, t2863: F, t2881: F, t2889: F, t2907: F, t4411: F, t933: F, t13550: F, t10296: F, t10298: F, t10302: F, t13644: F, t13630: F, t13632: F, t13635: F, t13638: F, t13640: F, t13642: F, t13647: F, t10300: F, t10784: F, t10785: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13552: F, t13557: F, t13561: F, t13616: F, t13624: F, t13626: F, t932: F, t4446: F, t942: F, t2929: F, t13716: F, t951: F, t10608: F, t324: F, t2924: F, t4475: F, t10756: F, t10820: F, t1581: F, t2856: F, t2900: F, t2925: F, t2930: F, t2933: F, t4434: F, t4449: F, t4472: F, t924: F, t943: F, t952: F, t10817: F, t4359: F, t10655: F, t4400: F, t4396: F, t912: F, t2792: F, t1557: F, t2836: F, t2793: F, t4399: F, t10661: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14235, t14238) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1556::<F>(t14228, t4337, t10408, t13510, t13512, t13514, t13517, t13519, t13522, t13524, t13526, t13657, t13661, t13665, t13720, t13722, t13726, t13729, t13731, t13734);
        let t14255 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1557::<F>(t13566, t13602, t10556, t10558, t10560, t10562, t10636, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
        let (t14257, t14262, t14263, t14266, t14271) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1558::<F>(t14255, t291, t10629, t1580, t10632, t2906, t959, t1573, t2904, t4408, t923, t1561, t2885);
        let t14279 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1559::<F>(t1561, t2860, t10760, t13517, t13519, t13522, t13524, t13526, t13657, t14263, t14266, t14271, t1569, t2863, t2881, t2889, t2907, t4411, t933);
        let (t14287, t14291, t14304) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1560::<F>(t13550, t13563, t10296, t10298, t10302, t13566, t13569, t13572, t13575, t13578, t13581, t13584, t13587);
        let t14328 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1561::<F>(t13644, t13602, t13598, t13613, t13630, t13632, t13635, t13638, t13640, t13642, t13647, t10300, t10556, t10558, t10560, t10562, t10784, t10785, t13530, t13534, t13539, t13544, t13548, t13552, t13557, t13561, t13616, t13624, t13626, t14287, t14291, t14304);
        let (t14329, t14332, t14337, t14344, t14363) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1562::<F>(t14328, t932, t4446, t942, t1573, t2929, t13716, t951, t13563, t13566, t13602, t10556, t10558, t10560, t10562, t10608, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
        let (t14364, t14373) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1563::<F>(t14363, t324, t2924, t4475, t10632, t1580, t2906, t10756, t10820, t13729, t14257, t14329, t14332, t14337, t14344, t1581, t2856, t2900, t2925, t2930, t2933, t4434, t4449, t4472, t924, t943, t952);
        let (t14376, t14378, t14381, t14384, t14387) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1564::<F>(t10817, t4359, t10655, t4400, t4396, t912, t2792, t1557, t2836, t2793, t4399, t10661);
    (t14235, t14238, t14257, t14262, t14279, t14364, t14373, t14376, t14378, t14381, t14384, t14387)
}
