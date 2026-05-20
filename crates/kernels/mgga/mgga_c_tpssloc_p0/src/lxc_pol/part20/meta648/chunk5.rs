//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2384/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2384<F: Float>(t47705: F, t47707: F, t47730: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t47732: F, t47736: F, t47738: F) -> F {
    let t48946 = F::new(8.0) / F::new(9.0) * t47705;
    let t48947 = F::new(8.0) / F::new(27.0) * t47707;
    let t48956 = F::new(4.0) / F::new(9.0) * t47730;
    let t48960 = -F::new(80.0) / F::new(81.0) * t47681 + F::new(4.0) * t47686 - F::new(2.0) / F::new(3.0) * t47691 - F::new(2.0) / F::new(3.0) * t47695 - F::new(2.0) / F::new(9.0) * t47699 - F::new(6.0) * t47703 + t48946 - t48947 + F::new(4.0) / F::new(9.0) * t47709 + F::new(2.0) / F::new(9.0) * t47711 + F::new(10.0) / F::new(27.0) * t47713 - F::new(4.0) / F::new(3.0) * t47715 - F::new(2.0) / F::new(3.0) * t47717 - F::new(10.0) / F::new(9.0) * t47722 - F::new(4.0) / F::new(3.0) * t47724 - F::new(8.0) * t47728 - t48956 + t47732 / F::new(3.0) - t47736 / F::new(3.0) + F::new(2.0) * t47738;
    t48960
}
