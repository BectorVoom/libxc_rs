//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1708/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1708<F: Float>(t12808: F, t656: F, t12747: F, t12750: F, t12752: F, t12754: F, t12758: F, t12761: F, t64: F, t9358: F, t9359: F, t9361: F, t9363: F) -> (F, F) {
    let t12809 = t656 * t12808;
    let t12812 = -t9358 - F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t9359 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9361 + t9363 / F::cast_from(3.0_f64) - F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t12747 - t12750 + t12752 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t64 * t12754 + t64 * t12758 / F::cast_from(2.0_f64) + t64 * t12761 / F::cast_from(4.0_f64) - t64 * t12809 / F::cast_from(8.0_f64);
    (t12809, t12812)
}
