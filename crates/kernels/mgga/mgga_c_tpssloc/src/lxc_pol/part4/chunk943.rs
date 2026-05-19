//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 943/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk943<F: Float>(t13783: F, t344: F, t4338: F, t2986: F, t10190: F, t4514: F, t10213: F, t60: F, t135: F, t340: F, t4548: F, t973: F) -> (F, F, F, F) {
    let t13784 = t13783 * t344;
    let t13785 = t13784 * t4338;
    let t13787 = F::cast_from(0.24691358024691358024e-3_f64) * t2986 * t13785;
    let t13788 = t10190 * t4514;
    let t13790 = F::cast_from(0.18518518518518518518e-3_f64) * t2986 * t13788;
    let t13797 = t60 * t10213;
    let t13798 = t13797 * t344;
    let t13822 = t135 * t340;
    let t13823 = t13822 * t4548;
    let t13825 = F::cast_from(0.55555555555555555554e-3_f64) * t973 * t13823;
    (t13787, t13790, t13798, t13825)
}
