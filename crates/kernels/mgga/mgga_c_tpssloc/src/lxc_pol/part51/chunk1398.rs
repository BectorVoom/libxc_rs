//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1398/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1398<F: Float>(t118588: F, t118596: F, t118602: F, t112818: F, t112821: F, t112830: F, t112847: F, t114732: F, t114734: F, t114737: F, t114739: F, t118586: F, t118590: F, t118592: F, t118594: F, t118606: F, t118608: F, t118610: F, t118612: F) -> F {
    let t121595 = F::new(7.0) / F::new(288.0) * t118588;
    let t121599 = F::new(7.0) / F::new(1152.0) * t118596;
    let t121601 = F::new(7.0) / F::new(1152.0) * t118602;
    let t121606 = F::new(0.26915170729426927235e-3) * t118586 + t121595 - t118590 / F::new(192.0) - t118592 / F::new(192.0) - t118594 / F::new(192.0) + t121599 + F::new(0.16149102437656156341e-2) * t112818 + t112821 + t112830 - t121601 + t114732 - t114734 - F::new(0.96894614625936938046e-2) * t118606 - t118608 / F::new(768.0) + t118610 / F::new(192.0) + t118612 / F::new(192.0) - t112847 + t114737 + t114739;
    t121606
}
