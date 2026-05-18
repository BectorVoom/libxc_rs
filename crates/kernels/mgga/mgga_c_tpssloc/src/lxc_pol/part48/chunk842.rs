//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 842/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk842<F: Float>(t794: F, t8537: F, t6562: F, t23237: F, t8547: F, t1880: F, t2053: F, t2717: F, t865: F, t23270: F, t1888: F, t857: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31319 = t794 * t8537;
    let t31320 = t6562 * t31319;
    let t31321 = F::new(0.41123351671205660912e-2) * t31320;
    let t31329 = t23237 * t8547;
    let t31330 = t1880 * t31329;
    let t31332 = t2717 * t2053;
    let t31333 = t31332 * t865;
    let t31334 = t23270 * t31333;
    let t31335 = t1888 * t31334;
    let t31337 = t857 * t2053;
    (t31319, t31321, t31329, t31330, t31332, t31333, t31334, t31335, t31337)
}
