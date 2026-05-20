//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1832;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta421<F: Float>(t14255: F, t291: F, t10629: F, t1580: F, t10632: F, t2906: F, t959: F, t1573: F, t2904: F, t4408: F, t923: F, t1561: F, t2885: F, t2860: F, t10760: F, t13517: F, t13519: F, t13522: F, t13524: F, t13526: F, t13657: F, t1569: F, t2863: F, t2881: F, t2889: F, t2907: F, t4411: F, t933: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14257, t14258, t14259, t14260, t14262, t14263, t14266, t14271) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1832::<F>(t14255, t291, t10629, t1580, t10632, t2906, t959, t1573, t2904, t4408, t923, t1561, t2885);
        let (t14276, t14279) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1833::<F>(t1561, t2860, t10760, t13517, t13519, t13522, t13524, t13526, t13657, t14263, t14266, t14271, t1569, t2863, t2881, t2889, t2907, t4411, t933);
    (t14257, t14258, t14259, t14260, t14262, t14263, t14266, t14271, t14276, t14279)
}
