//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1460/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1460<F: Float>(t2075: F, t26135: F, t652: F, t2314: F, t33620: F, t4034: F, t1458: F, t31518: F, t1873: F, t92090: F, t120908: F, t2039: F) -> (F, F, F, F, F, F) {
    let t122706 = F::new(2.0) * t652 * t2075 * t26135;
    let t122708 = F::new(2.0) * t2314 * t33620;
    let t122710 = F::new(2.0) * t4034 * t33620;
    let t122713 = F::new(2.0) * t652 * t31518 * t1458;
    let t122718 = t92090 * t1873;
    let t122719 = t120908 * t2039;
    (t122706, t122708, t122710, t122713, t122718, t122719)
}
