//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 953/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk953<F: Float>(t10789: F, t10804: F, t932: F, t2884: F, t922: F, t302: F, t2887: F, t310: F, t10743: F, t2791: F, t888: F, t2794: F) -> (F, F, F, F) {
    let t10805 = t10789 + t10804;
    let t10806 = t10805 * t932;
    let t10810 = F::new(1.0) / t2884 / t922;
    let t10811 = t302 * t10810;
    let t10813 = F::new(1.0) / t2887 / t310;
    let t10814 = t10743 * t10813;
    let t10817 = t888 * t2791;
    let t10819 = F::new(6.0) * t10817 * t2794;
    (t10806, t10811, t10814, t10819)
}
