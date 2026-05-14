//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 858/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk858<F: Float>(t75705: F, t1356: F, t37423: F, t8936: F, t14451: F, t5267: F, t26291: F, t5888: F, t40724: F, t75719: F, t75721: F, t75723: F, t75725: F, t69976: F, t69983: F, t71582: F, t75689: F, t75692: F, t75695: F, t75700: F, t75703: F, t75718: F) -> (F, F, F) {
    let t77782 = 0.44903406381989282115e-1 * t75705;
    let t77785 = 0.11974241701863808564e0 * t1356 * t37423 * t8936;
    let t77786 = t14451 * t5267;
    let t77787 = t26291 * t77786;
    let t77788 = 0.8980681276397856423e-1 * t77787;
    let t77789 = t14451 * t5888;
    let t77790 = t40724 * t77789;
    let t77791 = 0.8980681276397856423e-1 * t77790;
    let t77792 = 0.20455996240684006298e-1 * t75719;
    let t77793 = 0.2727466165424534173e-1 * t75721;
    let t77794 = 0.13637330827122670865e-1 * t75723;
    let t77795 = 0.44903406381989282115e-1 * t75725;
    let t77796 = 0.54549323308490683461e-1 * t69976;
    let t77797 = 0.72732431077987577948e-1 * t69983;
    let t77798 = -0.81756761766873046877e-6 * t75689 + 0.52557918278704101564e-6 * t75692 + 0.87596530464506835935e-6 * t75695 - 0.87596530464506835935e-6 * t75700 + 0.17519306092901367188e-6 * t75703 - t77782 - t77785 + t77788 + t77791 - t75718 - t77792 + t77793 + t77794 - t77795 + t77796 - t77797 + t71582;
    (t77786, t77789, t77798)
}
