//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1447/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1447<F: Float>(t24994: F, t8606: F, t24996: F, t2075: F, t26135: F, t652: F, t2314: F, t33620: F, t4034: F, t1458: F, t31518: F, t1873: F, t92090: F) -> (F, F, F, F, F, F) {
    let t122698 = t8606 * t24994;
    let t122700 = F::cast_from(6.0_f64) * t122698 * t24996;
    let t122706 = F::cast_from(2.0_f64) * t652 * t2075 * t26135;
    let t122708 = F::cast_from(2.0_f64) * t2314 * t33620;
    let t122710 = F::cast_from(2.0_f64) * t4034 * t33620;
    let t122713 = F::cast_from(2.0_f64) * t652 * t31518 * t1458;
    let t122718 = t92090 * t1873;
    (t122700, t122706, t122708, t122710, t122713, t122718)
}
