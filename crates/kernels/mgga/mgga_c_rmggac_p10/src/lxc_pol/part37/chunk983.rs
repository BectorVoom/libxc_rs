//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 983/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk983<F: Float>(t26291: F, t77786: F, t14451: F, t5888: F, t40724: F, t75719: F, t75721: F, t75723: F, t75725: F, t69976: F, t69983: F, t71583: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77787 = t26291 * t77786;
    let t77788 = F::cast_from(0.8980681276397856423e-1_f64) * t77787;
    let t77789 = t14451 * t5888;
    let t77790 = t40724 * t77789;
    let t77791 = F::cast_from(0.8980681276397856423e-1_f64) * t77790;
    let t77792 = F::cast_from(0.20455996240684006298e-1_f64) * t75719;
    let t77793 = F::cast_from(0.2727466165424534173e-1_f64) * t75721;
    let t77794 = F::cast_from(0.13637330827122670865e-1_f64) * t75723;
    let t77795 = F::cast_from(0.44903406381989282115e-1_f64) * t75725;
    let t77796 = F::cast_from(0.54549323308490683461e-1_f64) * t69976;
    let t77797 = F::cast_from(0.72732431077987577948e-1_f64) * t69983;
    let t77803 = F::cast_from(0.96056421943322389208e-3_f64) * t71583;
    (t77788, t77789, t77791, t77792, t77793, t77794, t77795, t77796, t77797, t77803)
}
