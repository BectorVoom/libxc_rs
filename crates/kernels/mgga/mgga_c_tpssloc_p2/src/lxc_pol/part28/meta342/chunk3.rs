//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1290/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1290<F: Float>(t12808: F, t656: F, t12747: F, t12750: F, t12752: F, t12754: F, t12758: F, t12761: F, t64: F, t9358: F, t9359: F, t9361: F, t9363: F) -> F {
    let t12809 = t656 * t12808;
    let t12812 = -t9358 - F::new(22.0) / F::new(9.0) * t9359 - F::new(2.0) / F::new(3.0) * t9361 + t9363 / F::new(3.0) - F::new(11.0) / F::new(9.0) * t12747 - t12750 + t12752 - F::new(3.0) / F::new(4.0) * t64 * t12754 + t64 * t12758 / F::new(2.0) + t64 * t12761 / F::new(4.0) - t64 * t12809 / F::new(8.0);
    t12812
}
