//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1285/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1285<F: Float>(t23012: F, t8557: F, t234: F, t7084: F, t112834: F, t112840: F, t112850: F, t112855: F, t31386: F, t6579: F, t8538: F, t31339: F, t81591: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114693 = t23012 * t8557;
    let t114694 = F::new(0.63969658155208805863e-1) * t114693;
    let t114696 = t234 * t7084;
    let t114732 = F::new(0.42167100809435519335e-2) * t112834;
    let t114734 = F::new(0.13457585364713463618e-3) * t112840;
    let t114737 = F::new(119.0) / F::new(3456.0) * t112850;
    let t114739 = F::new(0.90434973650874475512e-1) * t112855;
    let t114752 = t6579 * t31386;
    let t114759 = t23012 * t8538;
    let t114760 = F::new(0.63969658155208805863e-1) * t114759;
    let t114762 = t81591 * t31339;
    (t114694, t114696, t114732, t114734, t114737, t114739, t114752, t114760, t114762)
}
