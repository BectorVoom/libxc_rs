//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1214/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1214<F: Float>(t22852: F, t3792: F, t80786: F, t80798: F, t80749: F, t80751: F, t80753: F, t80755: F, t80757: F, t80759: F, t80761: F, t80763: F, t80767: F, t80769: F, t80773: F, t80776: F, t80780: F, t80784: F, t80789: F, t80792: F, t80794: F, t80796: F) -> F {
    let t80801 = t22852 * t80798 * t80786 * t3792;
    let t80803 = t80749 / F::cast_from(256.0_f64) - t80751 / F::cast_from(64.0_f64) + t80753 / F::cast_from(128.0_f64) - t80755 / F::cast_from(512.0_f64) - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t80757 + t80759 / F::cast_from(128.0_f64) + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t80761 - t80763 / F::cast_from(48.0_f64) - F::cast_from(0.2034786907144675699e0_f64) * t80767 + F::cast_from(0.25434836339308446238e-1_f64) * t80769 - F::cast_from(0.12111826828242117256e-2_f64) * t80773 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t80776 - F::cast_from(0.94875976821229918508e-2_f64) * t80780 + F::cast_from(0.50465945117675488567e-4_f64) * t80784 + F::cast_from(0.10093189023535097714e-3_f64) * t80789 - F::cast_from(0.15812662803538319751e-2_f64) * t80792 + F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t80794 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t80796 - F::cast_from(0.20186378047070195427e-3_f64) * t80801;
    t80803
}
