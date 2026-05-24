//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 961/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk961<F: Float>(t74456: F, t74459: F, t74465: F, t637: F, t8641: F, t71772: F, t8645: F, t71163: F, t8649: F, t71167: F, t70948: F, t74487: F) -> (F, F, F, F, F, F, F, F) {
    let t77014 = F::cast_from(0.5107751987195740728e-4_f64) * t74456;
    let t77015 = F::cast_from(0.1702583995731913576e-4_f64) * t74459;
    let t77017 = F::cast_from(0.15961724959986689775e-4_f64) * t74465;
    let t77018 = t637 * t8641;
    let t77019 = t71772 * t77018;
    let t77020 = F::cast_from(0.20455996240684006296e-1_f64) * t77019;
    let t77021 = t637 * t8645;
    let t77022 = t71163 * t77021;
    let t77023 = F::cast_from(0.40911992481368012592e-1_f64) * t77022;
    let t77024 = t637 * t8649;
    let t77025 = t71167 * t77024;
    let t77026 = F::cast_from(0.20455996240684006296e-1_f64) * t77025;
    let t77031 = F::cast_from(0.90915538847484472429e-2_f64) * t70948;
    let t77034 = F::cast_from(0.40911992481368012592e-1_f64) * t74487;
    (t77014, t77015, t77017, t77020, t77023, t77026, t77031, t77034)
}
