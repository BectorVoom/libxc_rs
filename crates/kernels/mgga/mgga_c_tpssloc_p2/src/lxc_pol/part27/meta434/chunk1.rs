//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1756/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1756<F: Float>(t1369: F, t22783: F, t3876: F, t6952: F, t3777: F, t6951: F, t6597: F, t6924: F, t281: F, t1307: F, t1361: F, t22690: F) -> (F, F, F, F, F, F, F, F) {
    let t22784 = t22783 * t1369;
    let t22785 = F::new(7.0) / F::new(288.0) * t22784;
    let t22786 = t6952 * t3876;
    let t22788 = t3777 * t6951;
    let t22789 = t22788 * t1369;
    let t22791 = t6597 * t6924;
    let t22792 = t22791 * t281;
    let t22794 = t22690 * t1361 * t1307;
    (t22784, t22785, t22786, t22788, t22789, t22791, t22792, t22794)
}
