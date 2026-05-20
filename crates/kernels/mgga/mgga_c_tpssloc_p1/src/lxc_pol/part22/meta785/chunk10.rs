//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2713/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2713<F: Float>(t12757: F, t19473: F, t19529: F, t20304: F, t20342: F, t2331: F, t29903: F, t4043: F, t4067: F, t45435: F, t5488: F, t55420: F, t64: F, t656: F, t666: F, t75592: F, t75601: F, t75603: F, t75613: F, t75657: F, t75694: F) -> F {
    let t75699 = t55420 + F::new(2.0) * t75592 + F::new(3.0) * t64 * t45435 * t20304 * t666 - F::new(9.0) / F::new(4.0) * t64 * t19473 * t4067 - F::new(2.0) * t75601 - F::new(9.0) / F::new(4.0) * t29903 * t75603 * t666 + F::new(3.0) / F::new(4.0) * t64 * t12757 * t5488 + F::new(3.0) / F::new(4.0) * t64 * t4043 * t19529 + t75613 / F::new(3.0) + t64 * t2331 * t20342 * t666 / F::new(4.0) - t64 * t656 * (t75657 + t75694) / F::new(8.0);
    t75699
}
