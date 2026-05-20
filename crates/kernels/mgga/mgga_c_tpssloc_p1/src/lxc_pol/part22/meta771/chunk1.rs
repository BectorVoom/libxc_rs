//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2626/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2626<F: Float>(t43859: F, t44466: F, t52313: F, t52339: F, t52343: F, t64074: F, t64076: F, t64087: F, t64089: F, t71470: F, t71472: F, t71474: F, t71477: F, t71480: F, t71483: F, t71486: F, t71489: F, t71505: F, t71508: F, t71511: F) -> F {
    let t73369 = -t52313 + F::new(4.0) / F::new(81.0) * t71470 - F::new(2.0) / F::new(9.0) * t71472 + F::new(2.0) / F::new(3.0) * t71474 - t71477 / F::new(3.0) + t71480 / F::new(6.0) + t71483 / F::new(6.0) - t71486 - t71489 - t44466 + F::new(40.0) / F::new(81.0) * t43859 + t71505 - F::new(3.0) * t71508 - F::new(2.0) / F::new(9.0) * t71511 - t52339 + t52343 - F::new(2.0) / F::new(9.0) * t64074 - F::new(2.0) / F::new(3.0) * t64076 + F::new(4.0) / F::new(3.0) * t64087 + F::new(2.0) * t64089;
    t73369
}
