//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1598/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1598<F: Float>(t11137: F, t11139: F, t11141: F, t11143: F, t11247: F, t14702: F, t14708: F, t14721: F, t14723: F, t14724: F, t14728: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F) -> F {
    let t14758 = -t11247 + F::new(8.0) / F::new(27.0) * t11137 + F::new(2.0) / F::new(27.0) * t11139 - F::new(2.0) / F::new(9.0) * t11141 - t11143 / F::new(9.0) + F::new(4.0) / F::new(27.0) * t14702 + t14721 - t14723 - t14724 + F::new(10.0) / F::new(27.0) * t14728 - F::new(4.0) / F::new(3.0) * t14733 - F::new(4.0) / F::new(9.0) * t14738 - F::new(2.0) / F::new(9.0) * t14742 + F::new(2.0) * t14746 + F::new(4.0) / F::new(3.0) * t14751 + F::new(2.0) / F::new(3.0) * t14755 + t14708 / F::new(3.0);
    t14758
}
