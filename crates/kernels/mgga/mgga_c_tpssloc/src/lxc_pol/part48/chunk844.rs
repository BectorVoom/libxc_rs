//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 844/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk844<F: Float>(t112850: F, t112855: F, t112823: F, t112825: F, t112827: F, t112829: F, t112832: F, t112837: F, t112843: F, t112853: F, t114732: F, t114734: F, t114736: F, t114726: F, t23035: F, t2379: F, t31376: F, t6637: F) -> (F, F) {
    let t114737 = 119.0 / 3456.0 * t112850;
    let t114739 = 0.90434973650874475512e-1 * t112855;
    let t114740 = -t112823 / 192.0 + 5.0 / 192.0 * t112825 - t112827 / 96.0 + 0.22608743412718618878e-1 * t112829 - 0.16149102437656156341e-2 * t112832 + t114732 + 0.19378922925187387609e-1 * t112837 - t114734 - 0.16149102437656156341e-2 * t112843 - t114736 + t114737 + t112853 / 768.0 + t114739;
    let t114741 = t114726 + t114740;
    let t114746 = t23035 * t6637 * t31376 * t2379;
    (t114741, t114746)
}
