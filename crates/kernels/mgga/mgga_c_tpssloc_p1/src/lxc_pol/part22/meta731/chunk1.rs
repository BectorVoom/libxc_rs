//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2398/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2398<F: Float>(t41904: F, t47787: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t59700: F, t59702: F, t59704: F, t59759: F, t59761: F, t68586: F, t68589: F, t68592: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F) -> F {
    let t68693 = F::new(2.0) * t68586 + F::new(2.0) / F::new(3.0) * t68589 - F::new(2.0) / F::new(9.0) * t68592 + F::new(40.0) / F::new(9.0) * t68596 - F::new(10.0) / F::new(9.0) * t68599 + F::new(4.0) * t68602 - F::new(10.0) / F::new(9.0) * t68605 - F::new(6.0) * t68608 - F::new(2.0) / F::new(3.0) * t59663 + F::new(2.0) / F::new(9.0) * t59665 + t59680 / F::new(3.0) + F::new(8.0) / F::new(9.0) * t59688 - F::new(4.0) / F::new(9.0) * t59694 + t41904 - F::new(4.0) / F::new(3.0) * t59700 + F::new(4.0) / F::new(9.0) * t59702 + F::new(10.0) / F::new(27.0) * t59704 + F::new(28.0) / F::new(27.0) * t47787 + F::new(2.0) * t59759 - F::new(4.0) / F::new(3.0) * t59761;
    t68693
}
