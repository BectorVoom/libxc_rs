//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1948;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1949;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta624<F: Float>(t26233: F, t3853: F, t1827: F, t80914: F, t1811: F, t80775: F, t7709: F, t80766: F, t22797: F, t5227: F, t22804: F, t26277: F, t16308: F, t22833: F, t16123: F, t2002: F, t559: F, t1307: F, t1377: F, t22633: F, t22635: F, t5353: F, t26215: F, t80650: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91391, t91394, t91398, t91400, t91402, t91404) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1948::<F>(t26233, t3853, t1827, t80914, t1811, t80775, t7709, t80766, t22797, t5227, t22804, t26277);
        let (t91413, t91416, t91449, t91455) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1949::<F>(t16308, t22833, t16123, t2002, t559, t1307, t1377, t22633, t22635, t5353, t26215, t80650);
    (t91391, t91394, t91398, t91400, t91402, t91404, t91413, t91416, t91449, t91455)
}
