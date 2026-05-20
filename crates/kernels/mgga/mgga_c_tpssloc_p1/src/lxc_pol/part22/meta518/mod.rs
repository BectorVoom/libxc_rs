//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta518<F: Float>(t22227: F, t22242: F, t475: F, t1214: F, t248: F, t21510: F, t4972: F, t4582: F, t11834: F, t1213: F, t1227: F, t15717: F, t15719: F, t15727: F, t15731: F, t15735: F, t1737: F, t1748: F, t18978: F, t18980: F, t18987: F, t19026: F, t19033: F, t19041: F, t19080: F, t22208: F, t22214: F, t22218: F, t5024: F, t6203: F, t6211: F) -> (F, F, F, F, F, F) {
        let (t22243, t22244, t22246, t22257, t22258, t22267) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1984::<F>(t22227, t22242, t475, t1214, t248, t21510, t4972, t4582, t11834, t1213, t1227, t15717, t15719, t15727, t15731, t15735, t1737, t1748, t18978, t18980, t18987, t19026, t19033, t19041, t19080, t22208, t22214, t22218, t5024, t6203, t6211);
    (t22243, t22244, t22246, t22257, t22258, t22267)
}
