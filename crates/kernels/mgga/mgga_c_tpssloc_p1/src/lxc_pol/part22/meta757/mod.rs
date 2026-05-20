//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta757 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2542;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta757<F: Float>(t1102: F, t21785: F, t43889: F, t18746: F, t4756: F, t14813: F, t5999: F, t71183: F, t71187: F, t71446: F, t71449: F, t71452: F, t71454: F, t71456: F, t71458: F, t18730: F, t4764: F, t21801: F, t699: F, t21788: F, t21791: F, t1113: F, t136: F, t71177: F, t3297: F, t71181: F, t71185: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t71461, t71463, t71465, t71467) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2542::<F>(t1102, t21785, t43889, t18746, t4756, t14813, t5999, t71183, t71187, t71446, t71449, t71452, t71454, t71456, t71458);
        let (t71468, t71470, t71472, t71474, t71477, t71480, t71483) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2543::<F>(t18730, t4764, t21801, t699, t21788, t21791, t1113, t136, t71177, t3297, t71181, t71185);
    (t71461, t71463, t71465, t71467, t71468, t71470, t71472, t71474, t71477, t71480, t71483)
}
