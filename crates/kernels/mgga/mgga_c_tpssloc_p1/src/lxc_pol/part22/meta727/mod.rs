//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta727 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta727<F: Float>(t20217: F, t2775: F, t607: F, t136: F, t908: F, t2770: F, t2826: F, t21118: F, t3966: F, t5677: F, t68481: F, t13541: F, t5398: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t68534, t68536, t68539, t68541, t68543, t68545, t68547, t68549, t68552, t68554) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2381::<F>(t20217, t2775, t607, t136, t908, t2770, t2826, t21118, t3966, t5677, t68481, t13541, t5398);
    (t68534, t68536, t68539, t68541, t68543, t68545, t68547, t68549, t68552, t68554)
}
