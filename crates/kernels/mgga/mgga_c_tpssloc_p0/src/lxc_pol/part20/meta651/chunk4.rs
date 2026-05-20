//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2398/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2398<F: Float>(t41831: F, t41833: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t49139: F) -> F {
    let t49140 = -F::cast_from(0.26837777777777777778e0_f64) * t47707 + F::cast_from(0.40256666666666666667e0_f64) * t47709 + F::cast_from(0.20128333333333333333e0_f64) * t47711 + F::cast_from(0.33547222222222222222e0_f64) * t47713 - F::new(0.12077e1) * t47715 - F::new(0.60385e0) * t47717 - F::cast_from(0.10064166666666666666e1_f64) * t47722 - F::new(0.12077e1) * t47724 - F::new(0.72462e1) * t47728 + F::cast_from(0.55190000000000000001e0_f64) * t41831 + F::new(0.33114e0) * t41833 - t49139;
    t49140
}
