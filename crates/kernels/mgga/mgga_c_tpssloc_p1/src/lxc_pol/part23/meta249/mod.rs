//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta249<F: Float>(t6014: F, t699: F, t6017: F, t135: F, t6146: F, t1174: F, t6140: F, t4889: F, t4916: F, t3403: F, t6084: F, t11285: F, t6068: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18505, t18512, t18529, t18530, t18532, t18533, t18536, t18615, t18622) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk908::<F>(t6014, t699, t6017, t135, t6146, t1174, t6140, t4889, t4916, t3403, t6084, t11285, t6068);
    (t18505, t18512, t18529, t18530, t18532, t18533, t18536, t18615, t18622)
}
