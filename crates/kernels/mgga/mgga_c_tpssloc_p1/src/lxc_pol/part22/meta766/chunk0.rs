//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2588/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2588<F: Float>(t22222: F, t3411: F, t14858: F, t6106: F, t1164: F, t18275: F, t21906: F, t44154: F, t21830: F, t6098: F, t22237: F, t71876: F, t71879: F, t72098: F, t72104: F, t72106: F) -> (F, F, F, F, F, F, F) {
    let t72201 = F::cast_from(0.35089341735807877242e1_f64) * t3411 * t22222;
    let t72203 = F::cast_from(0.51947577317044391276e2_f64) * t14858 * t6106;
    let t72207 = F::cast_from(0.12304822629859687989e5_f64) * t1164 * t44154 * t21906 * t18275;
    let t72209 = F::cast_from(0.51947577317044391277e2_f64) * t3411 * t21830;
    let t72211 = F::cast_from(0.35089341735807877242e1_f64) * t14858 * t6098;
    let t72213 = F::cast_from(0.10254018858216406658e4_f64) * t3411 * t22237;
    let t72214 = -t72098 - t71876 + t71879 - t72104 - t72106 - t72201 - t72203 + t72207 - t72209 + t72211 - t72213;
    (t72201, t72203, t72207, t72209, t72211, t72213, t72214)
}
