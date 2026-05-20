//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk945;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta215<F: Float>(t5726: F, t913: F, t893: F, t2844: F, t5694: F, t2842: F, t2848: F, t4335: F, t5679: F, t5683: F, t5687: F, t1568: F, t932: F, t2868: F, t2875: F, t4384: F, t5699: F, t5706: F, t5712: F, t5714: F, t5718: F, t5721: F, t5724: F) -> (F, F, F, F, F, F, F, F) {
        let (t5727, t5729, t5730, t5732, t5737, t5742) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk945::<F>(t5726, t913, t893, t2844, t5694, t2842, t2848, t4335, t5679, t5683, t5687, t1568);
        let (t5743, t5758) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk946::<F>(t5742, t932, t2868, t2875, t4335, t4384, t5679, t5683, t5687, t5699, t5706, t5712, t5714, t5718, t5721, t5724);
    (t5727, t5729, t5730, t5732, t5737, t5742, t5743, t5758)
}
