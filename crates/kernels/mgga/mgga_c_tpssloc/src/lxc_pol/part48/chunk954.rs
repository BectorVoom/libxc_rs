//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 954/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk954<F: Float>(t112834: F, t112840: F, t112846: F, t112850: F, t112855: F, t112823: F, t112825: F, t112827: F, t112829: F, t112832: F, t112837: F, t112843: F, t112853: F) -> F {
    let t114732 = F::cast_from(0.42167100809435519335e-2_f64) * t112834;
    let t114734 = F::cast_from(0.13457585364713463618e-3_f64) * t112840;
    let t114736 = F::new(7.0) / F::new(576.0) * t112846;
    let t114737 = F::new(119.0) / F::new(3456.0) * t112850;
    let t114739 = F::cast_from(0.90434973650874475512e-1_f64) * t112855;
    let t114740 = -t112823 / F::new(192.0) + F::new(5.0) / F::new(192.0) * t112825 - t112827 / F::new(96.0) + F::cast_from(0.22608743412718618878e-1_f64) * t112829 - F::cast_from(0.16149102437656156341e-2_f64) * t112832 + t114732 + F::cast_from(0.19378922925187387609e-1_f64) * t112837 - t114734 - F::cast_from(0.16149102437656156341e-2_f64) * t112843 - t114736 + t114737 + t112853 / F::new(768.0) + t114739;
    t114740
}
