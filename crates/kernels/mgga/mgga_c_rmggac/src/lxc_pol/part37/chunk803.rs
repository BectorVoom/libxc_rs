//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 803/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk803<F: Float>(t71772: F, t77018: F, t637: F, t8645: F, t71163: F, t8649: F, t71167: F, t70948: F, t74487: F, t2080: F, t739: F, t9530: F, t2010: F, t2012: F, t9343: F, t8901: F) -> (F, F, F, F, F, F, F, F) {
    let t77019 = t71772 * t77018;
    let t77020 = 0.20455996240684006296e-1 * t77019;
    let t77021 = t637 * t8645;
    let t77022 = t71163 * t77021;
    let t77023 = 0.40911992481368012592e-1 * t77022;
    let t77024 = t637 * t8649;
    let t77025 = t71167 * t77024;
    let t77026 = 0.20455996240684006296e-1 * t77025;
    let t77031 = 0.90915538847484472429e-2 * t70948;
    let t77034 = 0.40911992481368012592e-1 * t74487;
    let t77036 = t739 * t9530 * t2080;
    let t77037 = 0.2993560425465952141e-1 * t77036;
    let t77042 = t2010 * t2012 * t9343;
    let t77043 = 0.36021158228745895953e-3 * t77042;
    let t77044 = t637 * t8901;
    (t77020, t77023, t77026, t77031, t77034, t77037, t77043, t77044)
}
