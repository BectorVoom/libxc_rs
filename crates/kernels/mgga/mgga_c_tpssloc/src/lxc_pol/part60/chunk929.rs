//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 929/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk929<F: Float>(t114693: F, t112834: F, t112840: F, t112850: F, t112855: F, t23012: F, t8538: F, t2047: F, t213: F, t225: F, t794: F, t23030: F, t31405: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114694 = F::cast_from(0.63969658155208805863e-1_f64) * t114693;
    let t114732 = F::cast_from(0.42167100809435519335e-2_f64) * t112834;
    let t114734 = F::cast_from(0.13457585364713463618e-3_f64) * t112840;
    let t114737 = F::new(119.0) / F::new(3456.0) * t112850;
    let t114739 = F::cast_from(0.90434973650874475512e-1_f64) * t112855;
    let t114759 = t23012 * t8538;
    let t114760 = F::cast_from(0.63969658155208805863e-1_f64) * t114759;
    let t114770 = t213 * t2047 * t225;
    let t114790 = t794 * t2047;
    let t114814 = t23030 * t31405;
    (t114694, t114732, t114734, t114737, t114739, t114760, t114770, t114790, t114814)
}
