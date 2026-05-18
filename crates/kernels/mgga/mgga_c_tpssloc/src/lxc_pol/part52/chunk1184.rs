//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1184/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1184<F: Float>(t30988: F, t31828: F, t191: F, t192: F, t7412: F, t2020: F, t6997: F, t8690: F, t7000: F, t6535: F, t7266: F, t113: F, t1869: F, t30993: F, t30995: F, t31034: F, t31038: F, t31039: F, t31041: F, t7408: F, t8329: F) -> (F, F, F) {
    let t31829 = t31828 + t30988;
    let t31832 = t7412 * t191 * t192;
    let t31833 = t31832 * t2020;
    let t31834 = t8690 * t6997;
    let t31835 = t8690 * t7000;
    let t31838 = t7266 * t6535;
    let t31840 = -t113 * t31829 - t1869 * t7408 - t30993 - t30995 - t31034 - t31038 + F::new(3.0) * t31039 - t31041 + t31833 + t31834 - t31835 - F::new(2.0) * t31838 - t8329;
    (t31829, t31832, t31840)
}
