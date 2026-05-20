//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2244/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2244<F: Float>(t87198: F, t98610: F, t98612: F, t98614: F, t98616: F, t98618: F, t98620: F, t98622: F, t98624: F, t98626: F, t98629: F, t98631: F, t98633: F, t98635: F, t98637: F, t98639: F, t98642: F) -> F {
    let t98644 = t98610 / F::new(192.0) + t98612 / F::new(192.0) + t98614 / F::new(192.0) + t98616 / F::new(384.0) - F::new(7.0) / F::new(288.0) * t98618 + t98620 / F::new(256.0) + t98622 / F::new(768.0) - t87198 - t98624 / F::new(1536.0) - t98626 / F::new(256.0) + t98629 / F::new(384.0) - t98631 / F::new(192.0) + t98633 / F::new(384.0) + t98635 / F::new(384.0) - t98637 / F::new(768.0) - t98639 / F::new(1536.0) - F::cast_from(0.16956557559538964158e-1_f64) * t98642;
    t98644
}
