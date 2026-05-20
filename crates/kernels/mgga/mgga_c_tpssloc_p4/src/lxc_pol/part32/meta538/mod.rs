//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1879;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta538<F: Float>(t27551: F, t3961: F, t27550: F, t24826: F, t8074: F, t24788: F, t8066: F, t3247: F, t491: F, t24589: F, t24845: F, t24849: F, t27533: F, t27537: F, t27540: F, t27543: F, t27546: F, t27549: F, t3604: F, t3610: F, t3624: F, t7373: F, t8083: F) -> (F, F, F, F, F, F, F, F) {
        let (t27552, t27553, t27556, t27558, t27561, t27562, t27563, t27568) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1879::<F>(t27551, t3961, t27550, t24826, t8074, t24788, t8066, t3247, t491, t24589, t24845, t24849, t27533, t27537, t27540, t27543, t27546, t27549, t3604, t3610, t3624, t7373, t8083);
    (t27552, t27553, t27556, t27558, t27561, t27562, t27563, t27568)
}
