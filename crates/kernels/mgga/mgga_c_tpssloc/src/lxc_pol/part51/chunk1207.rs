//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1207/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1207<F: Float>(t33363: F, t6997: F, t114360: F, t120145: F, t120148: F, t120924: F, t120926: F, t120928: F, t120930: F, t120940: F, t120941: F, t120944: F, t120947: F, t2040: F, t26872: F, t27171: F, t33085: F, t6517: F, t7050: F) -> (F,) {
    let t120948 = t33363 * t6997;
    let t120951 = -3.0 * t114360 * t26872 - 2.0 * t120145 * t2040 - 2.0 * t120148 * t2040 - 2.0 * t27171 * t6517 - 2.0 * t33085 * t7050 - t120924 - t120926 - t120928 - t120930 + t120940 - t120941 + t120944 + t120947 + t120948;
    (t120951,)
}
