//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2816/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2816<F: Float>(t10143: F, t5660: F, t12895: F, t1877: F, t193: F, t202: F, t2522: F, t2749: F, t39585: F, t39590: F, t39593: F, t4119: F, t58139: F, t58973: F, t58974: F, t58975: F, t58978: F, t58979: F, t58980: F, t59434: F, t59475: F, t59518: F, t59558: F, t766: F, t870: F) -> F {
    let t59564 = t5660 * t10143;
    let t59571 = F::new(3.0) * t193 * t766 * t58139 + t193 * t202 * (t59434 + t59475 + t59518 + t59558) * t870 + t58973 - t39585 + t39590 + t58974 - t39593 + t58975 + t58978 + F::new(2.0) * t1877 * t59564 * t2749 + t58979 - t58980 + F::new(12.0) * t2522 * t12895 * t4119;
    t59571
}
