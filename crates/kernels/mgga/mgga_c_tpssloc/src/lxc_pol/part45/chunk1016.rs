//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1016/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1016<F: Float>(t114012: F, t114025: F, t114027: F, t114031: F, t114034: F, t114038: F, t114046: F, t114000: F, t114003: F, t114007: F, t114019: F, t114023: F, t114041: F) -> F {
    let t115458 = F::new(7.0) / F::new(576.0) * t114012;
    let t115461 = F::new(0.42167100809435519335e-2) * t114025;
    let t115462 = F::new(0.90434973650874475512e-1) * t114027;
    let t115463 = F::new(0.32298204875312312682e-2) * t114031;
    let t115464 = F::new(7.0) / F::new(576.0) * t114034;
    let t115465 = F::new(119.0) / F::new(3456.0) * t114038;
    let t115467 = F::new(0.5383034145885385447e-3) * t114046;
    let t115468 = F::new(0.13565246047631171327e0) * t114000 - t114003 / F::new(384.0) - t114007 / F::new(768.0) + t115458 + t114019 / F::new(384.0) - t114023 / F::new(768.0) + t115461 + t115462 + t115463 - t115464 + t115465 + t114041 / F::new(768.0) + t115467;
    t115468
}
