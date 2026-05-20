//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1971/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1971<F: Float>(t21898: F, t21991: F, t300: F, t1763: F, t6274: F, t11947: F, t193: F, t21726: F, t21728: F, t21730: F, t21732: F, t21812: F, t21815: F, t21829: F, t21832: F, t21835: F, t21897: F, t21901: F, t336: F) -> (F, F, F) {
    let t21993 = t300 * (t21898 + t21991);
    let t21994 = t6274 * t1763;
    let t21999 = F::new(2.0) * t11947 * t193 * t21994 * t336 + t21726 - t21728 - t21730 + t21732 + t21812 + t21815 + t21829 - t21832 + t21835 - t21897 + t21901 + t21993;
    (t21993, t21994, t21999)
}
