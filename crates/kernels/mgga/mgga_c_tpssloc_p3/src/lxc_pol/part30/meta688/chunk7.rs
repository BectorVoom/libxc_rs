//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2192/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2192<F: Float>(t25980: F, t4028: F, t12725: F, t7468: F, t2314: F, t28045: F, t4034: F, t19456: F, t24983: F, t25965: F, t7472: F, t97820: F, t97829: F, t97831: F, t97833: F, t97835: F, t97836: F, t97839: F, t97842: F, t97844: F, t97846: F, t97848: F) -> F {
    let t97850 = F::cast_from(4.0_f64) * t4028 * t25980;
    let t97854 = F::cast_from(4.0_f64) * t12725 * t7468;
    let t97856 = F::cast_from(4.0_f64) * t2314 * t28045;
    let t97858 = F::cast_from(4.0_f64) * t4034 * t28045;
    let t97859 = -F::cast_from(4.0_f64) * t12725 * t7472 - F::cast_from(4.0_f64) * t19456 * t7472 - F::cast_from(4.0_f64) * t24983 * t4028 - F::cast_from(4.0_f64) * t25965 * t4028 + t97820 - t97829 - t97831 - t97833 + t97835 - t97836 + t97839 + t97842 - t97844 - t97846 - t97848 - t97850 - t97854 - t97856 - t97858;
    t97859
}
