//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta245<F: Float>(t135: F, t6187: F, t1174: F, t4889: F, t5040: F, t6183: F, t6177: F, t248: F, t3570: F, t6225: F, t3506: F, t11697: F, t6191: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18324, t18325, t18327, t18329, t18330, t18332, t18333, t18356, t18357, t18371) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk904::<F>(t135, t6187, t1174, t4889, t5040, t6183, t6177, t248, t3570, t6225, t3506, t11697, t6191);
    (t18324, t18325, t18327, t18329, t18330, t18332, t18333, t18356, t18357, t18371)
}
