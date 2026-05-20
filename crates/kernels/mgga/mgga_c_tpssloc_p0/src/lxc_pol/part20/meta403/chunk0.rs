//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1800/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1800<F: Float>(t13784: F, t4338: F, t2986: F, t10190: F, t4514: F, t13528: F, t4510: F, t13532: F, t10213: F, t60: F, t344: F, t13537: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13785 = t13784 * t4338;
    let t13787 = F::cast_from(0.24691358024691358024e-3_f64) * t2986 * t13785;
    let t13788 = t10190 * t4514;
    let t13790 = F::cast_from(0.18518518518518518518e-3_f64) * t2986 * t13788;
    let t13791 = t4510 * t13528;
    let t13794 = t4510 * t13532;
    let t13797 = t60 * t10213;
    let t13798 = t13797 * t344;
    let t13799 = t13798 * t13537;
    (t13785, t13787, t13788, t13790, t13791, t13794, t13797, t13798, t13799)
}
