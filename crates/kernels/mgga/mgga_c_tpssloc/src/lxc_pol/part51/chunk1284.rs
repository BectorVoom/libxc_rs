//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1284/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1284<F: Float>(t114688: F, t31389: F, t6562: F, t794: F, t23012: F, t8557: F, t234: F, t7084: F, t112834: F, t112840: F, t112850: F, t112855: F) -> (F, F, F, F, F, F, F, F) {
    let t114689 = F::cast_from(0.82246703342411321824e-2_f64) * t114688;
    let t114691 = t6562 * t794 * t31389;
    let t114693 = t23012 * t8557;
    let t114694 = F::cast_from(0.63969658155208805863e-1_f64) * t114693;
    let t114696 = t234 * t7084;
    let t114732 = F::cast_from(0.42167100809435519335e-2_f64) * t112834;
    let t114734 = F::cast_from(0.13457585364713463618e-3_f64) * t112840;
    let t114737 = F::new(119.0) / F::new(3456.0) * t112850;
    let t114739 = F::cast_from(0.90434973650874475512e-1_f64) * t112855;
    (t114689, t114691, t114694, t114696, t114732, t114734, t114737, t114739)
}
