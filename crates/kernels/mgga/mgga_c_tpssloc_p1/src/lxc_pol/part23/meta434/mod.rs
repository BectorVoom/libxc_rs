//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1273;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta434<F: Float>(t15503: F, t18356: F, t18975: F, t5024: F, t1174: F, t21749: F, t3431: F, t135: F, t22011: F, t18375: F, t5019: F, t18329: F, t4889: F, t18324: F, t22136: F, t15740: F, t18371: F, t1222: F, t22175: F, t1734: F, t6218: F, t22169: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t72632, t72634, t72648, t72669, t72673, t72703) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1273::<F>(t15503, t18356, t18975, t5024, t1174, t21749, t3431, t135, t22011, t18375, t5019, t18329, t4889);
        let (t72705, t72708, t72727, t72733, t72767, t72798) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1274::<F>(t18324, t4889, t1174, t135, t22136, t15740, t18371, t1222, t22175, t1734, t6218, t22169);
    (t72632, t72634, t72648, t72669, t72673, t72703, t72705, t72708, t72727, t72733, t72767, t72798)
}
