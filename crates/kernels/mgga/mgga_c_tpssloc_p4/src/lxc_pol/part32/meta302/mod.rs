//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta302<F: Float>(t1053: F, t68: F, t1887: F, t337: F, t615: F, t134: F, t976: F, t984: F, t271: F, t2775: F, t974: F, t2769: F, t632: F) -> (F, F, F, F, F, F, F) {
        let (t10165, t10186, t10189, t10190, t10213, t10214, t10216) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1327::<F>(t1053, t68, t1887, t337, t615, t134, t976, t984, t271, t2775, t974, t2769, t632);
    (t10165, t10186, t10189, t10190, t10213, t10214, t10216)
}
