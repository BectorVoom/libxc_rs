//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 964/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk964<F: Float>(t2020: F, t31832: F, t6997: F, t8690: F, t7000: F, t6535: F, t7266: F, t8662: F, t9231: F, t9239: F, t131: F, t7245: F, t2240: F, t7254: F, t8301: F, t1873: F, t24932: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31833 = t31832 * t2020;
    let t31834 = t8690 * t6997;
    let t31835 = t8690 * t7000;
    let t31838 = t7266 * t6535;
    let t31857 = t9231 * t8662;
    let t31860 = t9239 * t8662;
    let t31863 = t7245 * t131;
    let t31864 = t2240 * t31863;
    let t31867 = t8301 * t7254;
    let t31868 = t2240 * t31867;
    let t31883 = t24932 * t1873;
    (t31833, t31834, t31835, t31838, t31857, t31860, t31864, t31868, t31883)
}
