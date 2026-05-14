//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1060/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1060<F: Float>(t81072: F, t81074: F, t80749: F, t80751: F, t80753: F, t80755: F, t80757: F, t80759: F, t80761: F, t80763: F, t80767: F, t80769: F, t80773: F, t80776: F, t80780: F, t80784: F, t80789: F, t80792: F, t80794: F, t80796: F, t80801: F) -> (F, F, F) {
    let t84480 = 0.55440370401180965083e0 * t81072;
    let t84481 = 0.3244175520728446583e0 * t81074;
    let t84508 = t80749 / 128.0 - t80751 / 32.0 + t80753 / 64.0 - t80755 / 256.0 - 5.0 / 64.0 * t80757 + t80759 / 64.0 + 7.0 / 24.0 * t80761 - t80763 / 24.0 - 0.4069573814289351398e0 * t80767 + 0.50869672678616892474e-1 * t80769 - 0.24223653656484234512e-2 * t80773 - 35.0 / 36.0 * t80776 - 0.18975195364245983701e-1 * t80780 + 0.10093189023535097713e-3 * t80784 + 0.20186378047070195427e-3 * t80789 - 0.31625325607076639502e-2 * t80792 + 119.0 / 1152.0 * t80794 - 7.0 / 384.0 * t80796 - 0.40372756094140390854e-3 * t80801;
    (t84480, t84481, t84508)
}
