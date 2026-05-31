//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1204/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1204<F: Float>(t193: F, t201: F, t7109: F, t10143: F, t82069: F, t2047: F, t2678: F, t81598: F, t81735: F, t81742: F, t81724: F, t81728: F, t81731: F, t81738: F, t81746: F, t81750: F, t81752: F, t81754: F, t81756: F, t81758: F, t81760: F, t81764: F, t81767: F, t81770: F, t81772: F, t81774: F, t81776: F) -> (F, F, F, F, F, F) {
    let t84797 = t193 * t201 * t7109;
    let t84800 = t7109 * t10143;
    let t84820 = F::cast_from(0.19739208802178717238e0_f64) * t82069;
    let t84842 = t2047 * t2678;
    let t84851 = F::cast_from(0.3244175520728446583e0_f64) * t81598;
    let t84857 = F::cast_from(0.13958506597733353653e-1_f64) * t81735;
    let t84859 = F::cast_from(0.87474304870637513515e-3_f64) * t81742;
    let t84873 = t81724 / F::cast_from(128.0_f64) - F::cast_from(0.14534192193890540707e-1_f64) * t81728 + F::cast_from(0.24223653656484234512e-2_f64) * t81731 - t84857 - F::cast_from(0.12111826828242117256e-2_f64) * t81738 + t84859 + F::cast_from(0.72670960969452703536e-2_f64) * t81746 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t81750 + t81752 / F::cast_from(64.0_f64) + t81754 / F::cast_from(64.0_f64) - t81756 / F::cast_from(32.0_f64) - t81758 / F::cast_from(256.0_f64) - t81760 / F::cast_from(64.0_f64) - F::cast_from(119.0_f64) / F::cast_from(288.0_f64) * t81764 - t81767 / F::cast_from(64.0_f64) + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t81770 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t81772 - t81774 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t81776;
    (t84797, t84800, t84820, t84842, t84851, t84873)
}
