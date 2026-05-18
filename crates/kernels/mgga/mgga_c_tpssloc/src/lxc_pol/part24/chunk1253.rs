//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1253/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1253<F: Float>(t22852: F, t3792: F, t80786: F, t80798: F, t80749: F, t80751: F, t80753: F, t80755: F, t80757: F, t80759: F, t80761: F, t80763: F, t80767: F, t80769: F, t80773: F, t80776: F, t80780: F, t80784: F, t80789: F, t80792: F, t80794: F, t80796: F) -> F {
    let t80801 = t22852 * t80798 * t80786 * t3792;
    let t80803 = t80749 / F::new(256.0) - t80751 / F::new(64.0) + t80753 / F::new(128.0) - t80755 / F::new(512.0) - F::new(5.0) / F::new(128.0) * t80757 + t80759 / F::new(128.0) + F::new(7.0) / F::new(48.0) * t80761 - t80763 / F::new(48.0) - F::new(0.2034786907144675699e0) * t80767 + F::new(0.25434836339308446238e-1) * t80769 - F::new(0.12111826828242117256e-2) * t80773 - F::new(35.0) / F::new(72.0) * t80776 - F::new(0.94875976821229918508e-2) * t80780 + F::new(0.50465945117675488567e-4) * t80784 + F::new(0.10093189023535097714e-3) * t80789 - F::new(0.15812662803538319751e-2) * t80792 + F::new(119.0) / F::new(2304.0) * t80794 - F::new(7.0) / F::new(768.0) * t80796 - F::new(0.20186378047070195427e-3) * t80801;
    t80803
}
