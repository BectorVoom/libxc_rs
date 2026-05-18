//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1280/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1280<F: Float>(t23127: F, t2703: F, t81724: F, t81728: F, t81731: F, t81736: F, t81738: F, t81743: F, t81746: F, t81750: F, t81752: F, t81754: F, t81756: F, t81758: F, t81760: F, t81764: F, t81767: F, t81770: F, t81772: F, t81774: F) -> F {
    let t81776 = t23127 * t2703;
    let t81778 = t81724 / F::new(256.0) - F::new(0.72670960969452703536e-2) * t81728 + F::new(0.12111826828242117256e-2) * t81731 - t81736 - F::new(0.60559134141210586281e-3) * t81738 + t81743 + F::new(0.36335480484726351768e-2) * t81746 - F::new(7.0) / F::new(96.0) * t81750 + t81752 / F::new(128.0) + t81754 / F::new(128.0) - t81756 / F::new(64.0) - t81758 / F::new(512.0) - t81760 / F::new(128.0) - F::new(119.0) / F::new(576.0) * t81764 - t81767 / F::new(128.0) + F::new(7.0) / F::new(96.0) * t81770 + F::new(7.0) / F::new(192.0) * t81772 - t81774 / F::new(384.0) + F::new(5.0) / F::new(128.0) * t81776;
    t81778
}
