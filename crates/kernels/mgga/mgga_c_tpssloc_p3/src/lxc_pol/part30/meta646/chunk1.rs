//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2060/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2060<F: Float>(t23384: F, t25827: F, t25436: F, t23328: F, t23394: F, t1054: F, t4693: F, t13783: F, t1926: F, t221: F, t25432: F, t25806: F, t6680: F) -> (F, F, F, F, F, F, F) {
    let t88753 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25827;
    let t88758 = F::cast_from(0.18277045187202515961e-2_f64) * t23384 * t25436;
    let t88772 = t23328 * t23394;
    let t88804 = t1054 * t4693;
    let t88810 = t1926 * t221 * t13783;
    let t88812 = F::cast_from(0.24369393582936687948e-2_f64) * t88810 * t25432;
    let t88845 = F::cast_from(0.14621636149762012769e-1_f64) * t6680 * t25806;
    (t88753, t88758, t88772, t88804, t88810, t88812, t88845)
}
