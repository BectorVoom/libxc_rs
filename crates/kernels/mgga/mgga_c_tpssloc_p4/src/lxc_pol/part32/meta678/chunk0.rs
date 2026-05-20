//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2116/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2116<F: Float>(t27553: F, t95772: F, t477: F, t5052: F, t27654: F, t7327: F, t24745: F, t4935: F, t24585: F, t7999: F, t24574: F, t27800: F) -> (F, F, F, F, F, F) {
    let t95774 = F::cast_from(0.24369393582936687948e-2_f64) * t95772 * t27553;
    let t95794 = t477 * t5052;
    let t95803 = t27654 * t7327;
    let t95813 = t4935 * t24745;
    let t95824 = t7999 * t24585;
    let t95834 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27800;
    (t95774, t95794, t95803, t95813, t95824, t95834)
}
