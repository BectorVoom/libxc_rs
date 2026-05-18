//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1431/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1431<F: Float>(t104735: F, t104787: F, t106804: F, t2110: F, t26016: F, t27298: F, t27937: F, t27979: F, t29475: F, t29478: F, t29481: F, t7428: F, t7975: F, t7978: F, t96473: F) -> F {
    let t108939 = -t106804 * t2110 / F::new(6.0) - t27937 * t7975 / F::new(2.0) - t27937 * t7978 / F::new(2.0) - t7428 * t29475 / F::new(2.0) - t7428 * t29478 - t7428 * t29481 / F::new(2.0) + t27979 * t7975 + t27979 * t7978 - F::new(5.0) * t96473 * t27298 - F::new(10.0) * t26016 * t104787 - F::new(10.0) * t26016 * t104735;
    t108939
}
