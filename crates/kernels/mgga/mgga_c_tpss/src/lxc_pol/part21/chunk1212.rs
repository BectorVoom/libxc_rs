//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1212/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1212<F: Float>(t1232: F, t12828: F, t19541: F, t1639: F, t520: F, t5731: F, t5745: F, t1768: F, t4459: F, t12823: F, t18497: F, t1265: F, t5740: F, t6255: F, t1773: F, t19497: F, t522: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19542 = t12828 * t1232;
    let t19543 = t19541 * t19542;
    let t19547 = t5731 * t1639 * t520;
    let t19548 = t5745 * t19547;
    let t19551 = t1768 * t4459 * t520;
    let t19552 = t5745 * t19551;
    let t19554 = t12823 * t520;
    let t19555 = t18497 * t19554;
    let t19559 = t5740 * t6255 * t1265;
    let t19563 = t6255 * t1232 * t520;
    let t19564 = t5745 * t19563;
    let t19567 = t1773 * t522 * t19497;
    (t19542, t19543, t19548, t19552, t19554, t19555, t19559, t19564, t19567)
}
