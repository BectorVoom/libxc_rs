//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 824/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk824<F: Float>(t75241: F, t15489: F, t16043: F, t1971: F, t3351: F, t44157: F, t875: F, t44183: F, t75250: F, t75254: F, t75262: F, t2211: F, t41091: F, t739: F, t41006: F, t884: F) -> (F, F, F, F, F, F, F, F, F) {
    let t77521 = 0.16263363996404810741e-4 * t75241;
    let t77528 = t16043 * t15489;
    let t77529 = 0.85129199786595678796e-5 * t77528;
    let t77532 = t3351 * t1971 * t875 * t44157;
    let t77533 = 0.85129199786595678796e-5 * t77532;
    let t77536 = t3351 * t1971 * t875 * t44183;
    let t77537 = 0.85129199786595678796e-5 * t77536;
    let t77540 = 0.60611291211334054834e-6 * t75250;
    let t77542 = 0.2727466165424534173e-1 * t75254;
    let t77545 = 0.23268647941669485538e-4 * t75262;
    let t77550 = 0.11974241701863808564e0 * t739 * t2211 * t41091;
    let t77553 = 0.11974241701863808564e0 * t884 * t2211 * t41006;
    (t77521, t77529, t77533, t77537, t77540, t77542, t77545, t77550, t77553)
}
