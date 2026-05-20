//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2003/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2003<F: Float>(t13170: F, t252: F, t1519: F, t2678: F, t13068: F, t225: F, t13030: F, t13062: F, t13378: F, t193: F, t2379: F, t14538: F) -> (F, F, F, F, F, F, F, F) {
    let t47448 = t252 * t13170;
    let t47528 = t1519 * t2678;
    let t47568 = t13068 * t225;
    let t47585 = t13030 * t225;
    let t47609 = t13062 * t225;
    let t47618 = t13378 * t225;
    let t47645 = t193 * t2379;
    let t50622 = t14538 * t225;
    (t47448, t47528, t47568, t47585, t47609, t47618, t47645, t50622)
}
