//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1168/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1168<F: Float>(t1656: F, t5731: F, t5740: F, t1768: F, t4516: F, t1265: F, t1640: F, t18497: F, t1258: F, t5570: F, t1771: F, t3255: F, t1232: F, t12828: F, t1639: F, t520: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19526 = t5731 * t1656;
    let t19527 = t5740 * t19526;
    let t19530 = t1768 * t4516;
    let t19531 = t5740 * t19530;
    let t19535 = t1640 * t1265;
    let t19536 = t18497 * t19535;
    let t19539 = t5570 * t1258;
    let t19540 = t1771 * t19539;
    let t19541 = t3255 * t1768;
    let t19542 = t12828 * t1232;
    let t19543 = t19541 * t19542;
    let t19547 = t5731 * t1639 * t520;
    (t19527, t19531, t19535, t19536, t19539, t19540, t19541, t19542, t19543, t19547)
}
