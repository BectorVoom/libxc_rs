//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 907/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk907<F: Float>(t1369: F, t22788: F, t6597: F, t6924: F, t281: F, t1307: F, t1361: F, t22690: F, t547: F, t6546: F, t1329: F, t3770: F, t6916: F, t2230: F, t213: F, t6928: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t22789 = t22788 * t1369;
    let t22791 = t6597 * t6924;
    let t22792 = t22791 * t281;
    let t22794 = t22690 * t1361 * t1307;
    let t22795 = t22792 * t22794;
    let t22797 = t6546 * t547;
    let t22798 = t22797 * t1329;
    let t22800 = t6916 * t3770;
    let t22803 = t2230 * t6924;
    let t22804 = t22803 * t213;
    let t22805 = t22804 * t6928;
    (t22789, t22791, t22792, t22794, t22795, t22797, t22798, t22800, t22803, t22804, t22805)
}
